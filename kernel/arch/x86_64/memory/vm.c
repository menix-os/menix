// Virtual memory management for x86.

#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/thread/process.h>
#include <menix/thread/spin.h>
#include <menix/util/log.h>
#include <menix/util/self.h>

#include <errno.h>
#include <string.h>

#define vm_flush_tlb(addr) asm volatile("invlpg (%0)" ::"r"(addr) : "memory")

SEGMENT_DECLARE_SYMBOLS(text)
SEGMENT_DECLARE_SYMBOLS(rodata)
SEGMENT_DECLARE_SYMBOLS(data)

static PageMap* kernel_map = NULL;									 // Page map used for the kernel.
static void* phys_base = NULL;										 // Memory mapped lower physical memory.
static VirtAddr kernel_foreign_base = CONFIG_vm_map_foreign_base;	 // Start of foreign mappings.

// If we can use the Supervisor Mode Access Prevention to run vm_hide_user() and vm_show_user()
bool can_smap = false;

void vm_init(void* base, PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries)
{
	phys_base = base;
	kassert(num_entries > 0, "No memory map entries given!");

	// Get a pointer to the first free physical memory page. Here we'll allocate our page directory structure.
	kernel_map = base + pm_alloc(1);
	kernel_map->lock = spin_new();
	kernel_map->head = base + pm_alloc(1);
	memset(kernel_map->head, 0x00, CONFIG_page_size);

	// Map all physical space.
	// Check for the highest usable physical memory address, so we know how much memory to map.
	usize highest = 0;
	for (usize i = 0; i < num_entries; i++)
	{
		const usize region_end = mem_map[i].address + mem_map[i].length;
		if (region_end > highest)
			highest = region_end;
	}

	for (usize cur = 0; cur < highest; cur += 2UL * MiB)
		kassert(vm_x86_map_page(kernel_map, cur, (VirtAddr)base + cur, PAGE_PRESENT | PAGE_READ_WRITE | PAGE_SIZE),
				"Unable to map lower memory!");

	// Map the kernel segments to the current physical address again.
	for (usize cur = (usize)SEGMENT_START(text); cur < (usize)SEGMENT_END(text); cur += CONFIG_page_size)
		kassert(vm_x86_map_page(kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, cur, PAGE_PRESENT),
				"Unable to map text segment!");

	for (usize cur = (usize)SEGMENT_START(rodata); cur < (usize)SEGMENT_END(rodata); cur += CONFIG_page_size)
		kassert(vm_x86_map_page(kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, cur,
								PAGE_PRESENT | PAGE_EXECUTE_DISABLE),
				"Unable to map rodata segment!");

	for (usize cur = (usize)SEGMENT_START(data); cur < (usize)SEGMENT_END(data); cur += CONFIG_page_size)
		kassert(vm_x86_map_page(kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, cur,
								PAGE_PRESENT | PAGE_READ_WRITE | PAGE_EXECUTE_DISABLE),
				"Unable to map data segment!");

	// If the physical base ever changes, update it in the physical memory manager as well.
	pm_update_phys_base(base);

	// Load the new page directory.

	asm_set_register(((usize)kernel_map->head - (usize)phys_base), cr3);
}

PageMap* vm_page_map_new()
{
	PageMap* result = kmalloc(sizeof(PageMap));
	result->lock = spin_new();
	// Allocate the first page table.
	usize* pt = pm_alloc(1) + pm_get_phys_base();
	memset(pt, 0, CONFIG_page_size);
	result->head = pt;

	// Copy over the upper half data which isn't accessible to user processes.
	// This way we don't have to swap page maps on a syscall.
	for (usize i = 256; i < 512; i++)
	{
		result->head[i] = kernel_map->head[i];
	}

	return result;
}

PageMap* vm_get_kernel_map()
{
	return kernel_map;
}

void vm_set_page_map(PageMap* page_map)
{
	asm_set_register(((VirtAddr)page_map->head - (VirtAddr)phys_base), cr3);
}

// Returns the next level of the current page map level. Optionally allocates a page.
static u64* vm_x86_traverse(u64* top, usize idx, bool allocate)
{
	// If we have allocated the next level, filter the address as offset and return the level.
	if (top[idx] & PAGE_PRESENT)
		return (u64*)(phys_base + (top[idx] & PAGE_ADDR));

	// If we don't want to allocate a page, but there was no page present, we can't do anything here.
	if (!allocate)
		return NULL;

	// Allocate the next level (will contain `CONFIG_page_size/sizeof(u64) == 512` entries).
	PhysAddr next_level = pm_alloc(1);
	memset(phys_base + next_level, 0, CONFIG_page_size);

	// Mark the next level as present so we don't allocate again.
	top[idx] = (u64)next_level | PAGE_PRESENT | PAGE_READ_WRITE;

	return (u64*)(phys_base + next_level);
}

static usize* vm_x86_get_pte(PageMap* page_map, VirtAddr virt_addr, bool allocate)
{
	kassert(page_map != NULL, "No page map was provided! Unable to get page table entry for 0x%p!", virt_addr);

	const usize virt_val = (usize)virt_addr;
	u64* cur_head = page_map->head;
	usize index = 0;

	for (usize lvl = 4; lvl >= 1; lvl--)
	{
		// Mask the respective bits for the address (9 bits per level, starting at bit 12).
		const usize shift = (12 + (9 * (lvl - 1)));
		// Index into the current level map.
		index = (virt_val >> shift) & 0x1FF;

		// If we allocate a 2MiB page, there is one less level in that page map branch.
		if (lvl == (cur_head[index] & PAGE_SIZE ? 2 : 1))
			break;

		// Update the head.
		cur_head = vm_x86_traverse(cur_head, index, allocate);
		if (cur_head == NULL)
		{
			spin_free(&page_map->lock);
			return NULL;
		}
	}

	return &cur_head[index];
}

static void destroy_level(u64* pml, usize start, usize end, u8 level)
{
	if (level == 0)
		return;

	for (usize i = start; i < end; i++)
	{
		u64* next_level = vm_x86_traverse(pml, i, false);
		destroy_level(next_level, 0, 512, level - 1);
	}

	pm_free((PhysAddr)pml - (PhysAddr)pm_get_phys_base(), 1);
}

void vm_page_map_destroy(PageMap* map)
{
	destroy_level(map->head, 0, 256, 4);
	kfree(map);
}

PageMap* vm_page_map_fork(PageMap* source)
{
	spin_acquire_force(&source->lock);
	PageMap* result = vm_page_map_new();

	if (result == NULL)
		goto fail;

fail:
	spin_free(&source->lock);
	if (result != NULL)
		vm_page_map_destroy(result);
	return result;
}

bool vm_x86_map_page(PageMap* page_map, VirtAddr phys_addr, VirtAddr virt_addr, usize flags)
{
	kassert(page_map != NULL, "No page map was provided! Unable to map page 0x%p to 0x%p!", phys_addr, virt_addr);

	spin_acquire_force(&page_map->lock);
	u64* cur_head = page_map->head;
	usize index = 0;

	for (usize lvl = 4; lvl >= 1; lvl--)
	{
		// Mask the respective bits for the address (9 bits per level, starting at bit 12).
		const usize shift = (12 + (9 * (lvl - 1)));
		// Index into the current level map.
		index = (virt_addr >> shift) & 0x1FF;

		// If we want to map a page for user mode access, we also have to map all previous levels for that.
		if (flags & PAGE_USER_MODE)
			cur_head[index] |= PAGE_USER_MODE;

		// If we allocate a 2MiB page, there is one less level in that page map branch.
		// In either case, don't traverse further after setting the index for writing.
		if (lvl == (flags & PAGE_SIZE ? 2 : 1))
			break;

		// Update the head.
		cur_head = vm_x86_traverse(cur_head, index, true);
		if (cur_head == NULL)
		{
			spin_free(&page_map->lock);
			return false;
		}
	}

	cur_head[index] = (phys_addr & PAGE_ADDR) | (flags & ~(PAGE_ADDR));
	spin_free(&page_map->lock);

	return true;
}

bool vm_x86_remap_page(PageMap* page_map, VirtAddr virt_addr, usize flags)
{
	kassert(page_map != NULL, "No page map was provided! Unable to remap page 0x%p to 0x%p!", phys_base, virt_addr);

	spin_acquire_force(&page_map->lock);
	u64* cur_head = page_map->head;
	usize index = 0;

	for (usize lvl = 4; lvl >= 1; lvl--)
	{
		// Mask the respective bits for the address (9 bits per level, starting at bit 12).
		const usize shift = (12 + (9 * (lvl - 1)));
		// Index into the current level map.
		index = (virt_addr >> shift) & 0x1FF;

		if (flags & PAGE_USER_MODE)
			cur_head[index] |= PAGE_USER_MODE;

		// If we allocate a 2MiB page, there is one less level in that page map branch.
		// In either case, don't traverse further after setting the index for writing.
		if (lvl == (flags & PAGE_SIZE ? 2 : 1))
			break;

		// Update the head.
		cur_head = vm_x86_traverse(cur_head, index, false);
		if (cur_head == NULL)
		{
			spin_free(&page_map->lock);
			return false;
		}
	}

	if ((cur_head[index] & PAGE_PRESENT) == 0)
	{
		spin_free(&page_map->lock);
		return false;
	}

	// Clear old flags.
	cur_head[index] &= PAGE_ADDR;
	// Set new ones.
	cur_head[index] |= (flags & ~(PAGE_ADDR));
	spin_free(&page_map->lock);

	return true;
}

bool vm_x86_unmap_page(PageMap* page_map, VirtAddr virt_addr)
{
	kassert(page_map != NULL, "No page map was provided! Unable to remap page 0x%p to 0x%p!", phys_base, virt_addr);

	spin_acquire_force(&page_map->lock);
	const usize virt_val = (usize)virt_addr;
	u64* cur_head = page_map->head;
	usize index = 0;

	for (usize lvl = 4; lvl >= 1; lvl--)
	{
		// Mask the respective bits for the address (9 bits per level, starting at bit 12).
		const usize shift = (12 + (9 * (lvl - 1)));
		// Index into the current level map.
		index = (virt_val >> shift) & 0x1FF;

		if (lvl == 1)
			break;

		// Update the head.
		cur_head = vm_x86_traverse(cur_head, index, false);
		if (cur_head == NULL)
		{
			spin_free(&page_map->lock);
			return false;
		}
	}

	if ((cur_head[index] & PAGE_PRESENT) == 0)
	{
		spin_free(&page_map->lock);
		return false;
	}

	// Clear everything.
	cur_head[index] = 0;
	spin_free(&page_map->lock);

	return true;
}

PhysAddr vm_virt_to_phys(PageMap* page_map, VirtAddr address)
{
	spin_acquire_force(&page_map->lock);
	usize* pte = vm_x86_get_pte(page_map, address, false);
	spin_free(&page_map->lock);

	// If the page is not present or the entry doesn't exist, we can't return a physical address.
	if (pte == NULL || (*pte & PAGE_PRESENT) == false)
		return ~0UL;

	return (*pte) & 0xFFFFFFFFFF000;
}

// Converts POSIX protection flags to x86 page flags
static PageFlags vm_posix_prot_to_x86(PageMap* page_map, int prot)
{
	PageFlags x86_flags = PAGE_PRESENT;
	if (page_map != kernel_map)
		x86_flags |= PAGE_USER_MODE;
	if (prot & PROT_WRITE)
		x86_flags |= PAGE_READ_WRITE;
	if ((prot & PROT_EXEC) == 0)
		x86_flags |= PAGE_EXECUTE_DISABLE;

	return x86_flags;
}

VirtAddr vm_map(PageMap* page_map, VirtAddr hint, usize length, usize prot, int flags, Handle* fd, usize off)
{
	if (length == 0)
	{
		thread_errno = EINVAL;
		return (VirtAddr)MAP_FAILED;
	}

	// Convert flags to x86 page flags.
	PageFlags x86_flags = vm_posix_prot_to_x86(page_map, prot);

	VirtAddr addr = 0;
	length = ALIGN_UP(length, CONFIG_page_size);
	usize page_count = length / CONFIG_page_size;
	VirtAddr aligned_hint = ALIGN_DOWN(hint, CONFIG_page_size);

	Thread* thread = arch_current_cpu()->thread;
	Process* proc = thread->parent;

	// Check the hint and make changes if necessary.
	if (flags & MAP_FIXED)
	{
		// Check if there already is a mapping at the hinted address.
		// If there is not, we can take the hint as is.
		if (!vm_unmap(page_map, aligned_hint, length) && (flags & MAP_FIXED))
			return (VirtAddr)MAP_FAILED;

		// Check if we're mapping between pages. If yes, we need one more page.
		if (aligned_hint < hint)
			page_count += 1;

		addr = hint;
	}
	else
	{
		// Choose the next free region of virtual memory if no hint was given.
		addr = proc->map_base;
	}

	// TODO: The map_base should only be relevant when not doing a MAP_FIXED.
	// TODO: This might waste a ton of available virtual address space!
	proc->map_base += CONFIG_page_size * page_count;

	for (usize i = 0; i < page_count; i++)
	{
		PhysAddr page = pm_alloc(1);
		if (vm_x86_map_page(page_map, page, addr + (i * CONFIG_page_size), x86_flags) == false)
		{
			pm_free(page, 1);
			return (VirtAddr)MAP_FAILED;
		}
	}

	return addr;
}

bool vm_protect(PageMap* page_map, VirtAddr virt_addr, usize length, usize prot)
{
	const usize page_count = ALIGN_UP(length, CONFIG_page_size) / CONFIG_page_size;
	usize x86_flags = vm_posix_prot_to_x86(page_map, prot);

	for (usize i = 0; i < page_count; i++)
		vm_x86_remap_page(page_map, virt_addr + (i * CONFIG_page_size), x86_flags);

	vm_flush_tlb(virt_addr);
	return true;
}

bool vm_unmap(PageMap* page_map, VirtAddr virt_addr, usize length)
{
	// TODO
	vm_flush_tlb(virt_addr);
	return true;
}

void* vm_map_foreign(PageMap* page_map, VirtAddr foreign_addr, usize num_pages)
{
	VirtAddr start = kernel_foreign_base;

	for (usize page = 0; page < num_pages; page++)
	{
		if (vm_x86_map_page(kernel_map, vm_virt_to_phys(page_map, foreign_addr + (page * CONFIG_page_size)),
							start + (page * CONFIG_page_size), PAGE_READ_WRITE | PAGE_PRESENT) == false)
		{
			return MAP_FAILED;
		}
	}

	kernel_foreign_base += num_pages * CONFIG_page_size;

	return (void*)start;
}

bool vm_unmap_foreign(void* kernel_addr, usize num_pages)
{
	for (usize page = 0; page < num_pages; page++)
	{
		if (vm_x86_unmap_page(kernel_map, (VirtAddr)kernel_addr + (page * CONFIG_page_size)) == false)
			return false;
	}
	return true;
}

void vm_hide_user()
{
	if (can_smap)
		asm volatile("clac");
}

void vm_show_user()
{
	if (can_smap)
		asm volatile("stac");
}

void interrupt_pf_handler(CpuRegisters* regs)
{
	// CR2 holds the address that was accessed.
	usize cr2;
	asm_get_register(cr2, cr2);

#if !defined(NDEBUG) && defined(CONFIG_x86_pf_debug)
	kmesg("Page fault: \n");

	// Present
	if (BIT(regs->error, 0))
	{
		kmesg("\t- Fault was a protection violation, permissions are: PAGE_READ ");
		usize flags = *vm_x86_get_pte(proc->page_map, cr2, false) & ~PAGE_ADDR;
		if (flags & PAGE_READ_WRITE)
			kmesg("| PAGE_WRITE");
		if (flags & PAGE_EXECUTE_DISABLE)
			kmesg("| PAGE_EXECUTE_DISABLE");
		kmesg("\n");
	}
	else
		kmesg("\t- Page was not present\n");

	// Write
	if (BIT(regs->error, 1))
		kmesg("\t- Fault was caused by a write access\n");
	else
		kmesg("\t- Fault was caused by a read access\n");

	// User
	if (BIT(regs->error, 2))
		kmesg("\t- Fault was caused by the user\n");
	else
		kmesg("\t- Fault was caused by the kernel\n");

	// Instruction fetch
	if (BIT(regs->error, 4))
		kmesg("\t- Fault was caused by an instruction fetch\n");

	// Check if SMAP is blocking this access
	if (can_smap & !BIT(regs->rflags, 18) & !(regs->cs & CPL_USER))
		kmesg("\t- Fault was caused by SMAP (missing vm_show_user()?)\n");

	kmesg("Attempted to access 0x%p!\n", cr2);
	ktrace();
#endif

	// If the current protection level wasn't 3, that means the page fault was
	// caused by the supervisor! If this happens, we messed up big time!
	if (!(regs->cs & CPL_USER))
	{
		// If we can't recover, abort.
		kmesg("Fatal page fault in kernel mode while trying to access 0x%p! (Error: 0x%zx)\n", cr2, regs->error);
		kabort();
	}

	// TODO: Handle user page fault.
	if (arch_current_cpu())
	{
		Process* proc = arch_current_cpu()->thread->parent;

		// If nothing can make the process recover, we have to put it out of its misery.
		process_kill(proc, true);
		kmesg("PID %zu terminated with SIGSEGV.\n", proc->id);
	}
}
