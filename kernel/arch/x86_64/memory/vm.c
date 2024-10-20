// Virtual memory management for x86.

#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/thread/process.h>
#include <menix/thread/spin.h>
#include <menix/util/log.h>
#include <menix/util/self.h>

#include <string.h>

#define vm_flush_tlb(addr) asm volatile("invlpg (%0)" ::"r"(addr) : "memory")

SEGMENT_DECLARE_SYMBOLS(text)
SEGMENT_DECLARE_SYMBOLS(rodata)
SEGMENT_DECLARE_SYMBOLS(data)

PageMap* kernel_map = NULL;									  // Page map used for the kernel.
VirtAddr kernel_foreign_base = CONFIG_vm_map_foreign_base;	  // Start of foreign mappings.

// If we can use the Supervisor Mode Access Prevention to run vm_hide_user() and vm_show_user()
bool can_smap = false;

void vm_init(PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries)
{
	kassert(num_entries > 0, "No memory map entries given!");

	// Get a pointer to the first free physical memory page. Here we'll allocate our page directory structure.
	kernel_map = pm_get_phys_base() + pm_alloc(1);
	kernel_map->lock = spin_new();
	kernel_map->head = pm_get_phys_base() + pm_alloc(1);
	memset(kernel_map->head, 0x00, arch_page_size);

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
		kassert(
			vm_x86_map(kernel_map, cur, (VirtAddr)pm_get_phys_base() + cur, PAGE_PRESENT | PAGE_READ_WRITE | PAGE_SIZE),
			"Unable to map lower memory!");

	// Map the kernel segments to the current physical address again.
	for (usize cur = (usize)SEGMENT_START(text); cur < (usize)SEGMENT_END(text); cur += arch_page_size)
		kassert(vm_map(kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, cur, VMProt_Read | VMProt_Execute, 0),
				"Unable to map text segment!");

	for (usize cur = (usize)SEGMENT_START(rodata); cur < (usize)SEGMENT_END(rodata); cur += arch_page_size)
		kassert(vm_map(kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, cur, VMProt_Read, 0),
				"Unable to map rodata segment!");

	for (usize cur = (usize)SEGMENT_START(data); cur < (usize)SEGMENT_END(data); cur += arch_page_size)
		kassert(vm_map(kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, cur, VMProt_Read | VMProt_Write, 0),
				"Unable to map data segment!");

	// Load the new page directory.
	asm_set_register(((usize)kernel_map->head - (usize)pm_get_phys_base()), cr3);
}

PageMap* vm_page_map_new()
{
	PageMap* result = kmalloc(sizeof(PageMap));
	result->lock = spin_new();
	// Allocate the first page table.
	usize* pt = pm_alloc(1) + pm_get_phys_base();
	memset(pt, 0, PAGE_SIZE);
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
	asm_set_register(((VirtAddr)page_map->head - (VirtAddr)pm_get_phys_base()), cr3);
}

// Returns the next level of the current page map level. Optionally allocates a page.
static u64* vm_x86_traverse(u64* top, usize idx, bool allocate)
{
	// If we have allocated the next level, filter the address as offset and return the level.
	if (top[idx] & PAGE_PRESENT)
		return (u64*)(pm_get_phys_base() + (top[idx] & PAGE_ADDR));

	// If we don't want to allocate a page, but there was no page present, we can't do anything here.
	if (!allocate)
		return NULL;

	// Allocate the next level (will contain `PAGE_SIZE/sizeof(u64) == 512` entries).
	PhysAddr next_level = pm_alloc(1);
	memset(pm_get_phys_base() + next_level, 0, arch_page_size);

	// Mark the next level as present so we don't allocate again.
	top[idx] = (u64)next_level | PAGE_PRESENT | PAGE_READ_WRITE;

	return (u64*)(pm_get_phys_base() + next_level);
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

bool vm_x86_map(PageMap* page_map, VirtAddr phys_addr, VirtAddr virt_addr, usize flags)
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

bool vm_x86_remap(PageMap* page_map, VirtAddr virt_addr, usize flags)
{
	kassert(page_map != NULL, "No page map was provided! Unable to remap page 0x%p to 0x%p!", pm_get_phys_base(),
			virt_addr);

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

bool vm_x86_unmap(PageMap* page_map, VirtAddr virt_addr)
{
	kassert(page_map != NULL, "No page map was provided! Unable to remap page 0x%p to 0x%p!", pm_get_phys_base(),
			virt_addr);

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

bool vm_is_mapped(PageMap* page_map, VirtAddr address, VMProt prot)
{
	PhysAddr phys = vm_virt_to_phys(page_map, address);

	// Address is not mapped at all.
	if (phys == ~0UL)
		return false;

	return true;
}

// Converts protection flags to x86 page flags.
static PageFlags vm_flags_to_x86(PageMap* page_map, VMProt prot, VMFlags flags)
{
	PageFlags x86_flags = PAGE_PRESENT;

	if (page_map != kernel_map)
		x86_flags |= PAGE_USER_MODE;
	if (prot & VMProt_Write)
		x86_flags |= PAGE_READ_WRITE;
	if ((prot & VMProt_Execute) == 0)
		x86_flags |= PAGE_EXECUTE_DISABLE;

	return x86_flags;
}

bool vm_map(PageMap* page_map, PhysAddr phys_addr, VirtAddr virt_addr, VMProt prot, VMFlags flags)
{
	kassert(page_map != NULL, "No page map provided!");
	kassert(phys_addr % arch_page_size == 0, "Physical address is not page aligned! Value: %zu", phys_addr);

	PageFlags x86_flags = vm_flags_to_x86(page_map, prot, flags);
	return vm_x86_map(page_map, phys_addr, virt_addr, x86_flags);
}

bool vm_protect(PageMap* page_map, VirtAddr virt_addr, VMProt prot)
{
	usize x86_flags = vm_flags_to_x86(page_map, prot, 0);
	bool result = vm_x86_remap(page_map, virt_addr, x86_flags);
	if (result == true)
		vm_flush_tlb(virt_addr);
	return result;
}

bool vm_unmap(PageMap* page_map, VirtAddr virt_addr)
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
		if (vm_x86_map(kernel_map, vm_virt_to_phys(page_map, foreign_addr + (page * PAGE_SIZE)),
					   start + (page * PAGE_SIZE), PAGE_READ_WRITE | PAGE_PRESENT) == false)
		{
			return MAP_FAILED;
		}
	}

	kernel_foreign_base += num_pages * PAGE_SIZE;

	return (void*)start;
}

bool vm_unmap_foreign(void* kernel_addr, usize num_pages)
{
	for (usize page = 0; page < num_pages; page++)
	{
		if (vm_x86_unmap(kernel_map, (VirtAddr)kernel_addr + (page * PAGE_SIZE)) == false)
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

void interrupt_pf_handler(Context* regs)
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
