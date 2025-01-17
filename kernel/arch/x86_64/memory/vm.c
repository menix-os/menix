// Virtual memory management for x86.

#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/util/log.h>
#include <menix/util/self.h>
#include <menix/util/spin.h>

#include <string.h>

#define vm_flush_tlb(addr) asm volatile("invlpg (%0)" ::"r"(addr) : "memory")

#define PT_PRESENT		   (1 << 0)
#define PT_READ_WRITE	   (1 << 1)
#define PT_USER_MODE	   (1 << 2)
#define PT_WRITE_THROUGH   (1 << 3)
#define PT_CACHE_DISABLE   (1 << 4)
#define PT_ACCESSED		   (1 << 5)
#define PT_DIRTY		   (1 << 6)
#define PT_SIZE			   (1 << 7)
#define PT_GLOBAL		   (1 << 8)
#define PT_AVAILABLE	   (1 << 9)
#define PT_ATTRIBUTE_TABLE (1 << 10)
#define PT_EXECUTE_DISABLE (1UL << 63)
#define PT_ADDR_MASK	   (0x0000FFFFFFFFF000UL)

// If we can use the Supervisor Mode Access Prevention.
bool can_smap = false;

void vm_user_show()
{
	if (can_smap)
		asm volatile("stac");
}

void vm_user_hide()
{
	if (can_smap)
		asm volatile("clac");
}

PageMap* vm_page_map_new()
{
	PageMap* result = kmalloc(sizeof(PageMap));
	result->lock = (SpinLock) {0};

	// Allocate the first page table.
	usize* pt = pm_alloc(1) + pm_get_phys_base();
	memset(pt, 0, arch_page_size);
	result->head = pt;

	// Copy over the upper half data which isn't accessible to user processes.
	for (usize i = 256; i < 512; i++)
	{
		result->head[i] = vm_kernel_map->head[i];
	}

	return result;
}

void vm_set_page_map(PageMap* page_map)
{
	asm_set_register(((VirtAddr)page_map->head - (VirtAddr)pm_get_phys_base()), cr3);
}

// Returns the next level of the current page map level. Optionally allocates a page.
static u64* vm_x86_traverse(u64* top, usize idx, bool allocate)
{
	// If we have allocated the next level, filter the address as offset and return the level.
	if (top[idx] & PT_PRESENT)
		return (u64*)(pm_get_phys_base() + (top[idx] & PT_ADDR_MASK));

	// If we don't want to allocate a page, but there was no page present, we can't do anything here.
	if (!allocate)
		return NULL;

	// Allocate the next level (will contain `arch_page_size/sizeof(u64) == 512` entries).
	PhysAddr next_level = pm_alloc(1);
	memset(pm_get_phys_base() + next_level, 0, arch_page_size);

	// Mark the next level as present so we don't allocate again.
	usize flags = top[idx] | PT_PRESENT | PT_READ_WRITE;
	top[idx] = (u64)(next_level & PT_ADDR_MASK) | (flags & ~(PT_ADDR_MASK));

	return (u64*)(pm_get_phys_base() + next_level);
}

static usize* vm_x86_get_pte(PageMap* page_map, VirtAddr virt_addr, bool allocate)
{
	kassert(page_map != NULL, "No page map was provided! Unable to get page table entry for 0x%p!", virt_addr);

	const usize virt_val = (usize)virt_addr;
	u64* cur_head = page_map->head;
	usize index = 0;

	bool do_break = false;
	for (usize lvl = 4; lvl >= 1; lvl--)
	{
		// Mask the respective bits for the address (9 bits per level, starting at bit 12).
		const usize shift = (12 + (9 * (lvl - 1)));
		// Index into the current level map.
		index = (virt_val >> shift) & 0x1FF;

		if (do_break || lvl == 1)
			break;

		// If we allocate a 2MiB page, there is one less level in that page map branch.
		if (cur_head[index] & PT_SIZE)
			do_break = true;

		// Update the head.
		cur_head = vm_x86_traverse(cur_head, index, allocate);
		if (cur_head == NULL)
		{
			spin_unlock(&page_map->lock);
			return NULL;
		}
	}

	return &cur_head[index];
}

static void destroy_level(u64* pml, usize start, usize end, u8 level)
{
	if (level == 0 || pml == NULL)
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
	kassert(source != NULL, "No page map was provided! Unable to fork page map!");

	PageMap* result = vm_page_map_new();
	if (result == NULL)
		return NULL;

	// TODO

	spin_lock(&source->lock);
	const usize page_size = vm_get_page_size(VMLevel_Small);
	// Copy all existing mappings and copy underlying memory to the new pages.
	list_iter(&source->maps, iter)
	{
		// Allocate new backing memory.
		PhysAddr new_phys = pm_alloc(iter->num_pages);
		// Create a window for us to copy into.
		void* mem = vm_map_memory(new_phys, iter->num_pages * page_size, VMProt_Read | VMProt_Write);
		void* source_mem = vm_map_foreign(source, iter->virtual, iter->num_pages);
		if (source_mem == NULL)
			goto fail;

		// Map the new pages into the existing mappings.
		for (usize i = 0; i < iter->num_pages; i++)
		{
			const usize offset = i * page_size;
			if (!vm_map(result, new_phys + offset, iter->virtual + offset, iter->prot, iter->flags, VMLevel_Small))
			{
				pm_free(new_phys, iter->num_pages);
				goto fail;
			}
			// Copy old page memory to the new page.
			memcpy(mem + offset, source_mem + offset, page_size);
		}
	}

fail:
	spin_unlock(&source->lock);
	if (result != NULL)
		vm_page_map_destroy(result);
	return result;
}

bool vm_x86_remap(PageMap* page_map, VirtAddr virt_addr, usize flags)
{
	kassert(page_map != NULL, "No page map was provided! Unable to remap page 0x%p!", virt_addr);

	spin_lock(&page_map->lock);
	u64* cur_head = page_map->head;
	usize index = 0;

	for (usize lvl = 4; lvl >= 1; lvl--)
	{
		// Mask the respective bits for the address (9 bits per level, starting at bit 12).
		const usize shift = (12 + (9 * (lvl - 1)));
		// Index into the current level map.
		index = (virt_addr >> shift) & 0x1FF;

		if (flags & PT_USER_MODE)
			cur_head[index] |= PT_USER_MODE;

		// If we allocate a 2MiB page, there is one less level in that page map branch.
		// In either case, don't traverse further after setting the index for writing.
		if (lvl == (flags & PT_SIZE ? 2 : 1))
			break;

		// Update the head.
		cur_head = vm_x86_traverse(cur_head, index, false);
		if (cur_head == NULL)
		{
			spin_unlock(&page_map->lock);
			return false;
		}
	}

	if ((cur_head[index] & PT_PRESENT) == 0)
	{
		spin_unlock(&page_map->lock);
		return false;
	}

	// Clear old flags.
	cur_head[index] &= PT_ADDR_MASK;
	// Set new ones.
	cur_head[index] |= (flags & ~(PT_ADDR_MASK));
	spin_unlock(&page_map->lock);

	return true;
}

bool vm_unmap(PageMap* page_map, VirtAddr virt_addr)
{
	kassert(page_map != NULL, "No page map was provided! Unable to remap page 0x%p to 0x%p!", pm_get_phys_base(),
			virt_addr);

	spin_lock(&page_map->lock);
	const usize virt_val = (usize)virt_addr;
	u64* cur_head = page_map->head;
	usize index = 0;

	for (usize lvl = 4; lvl >= 1; lvl--)
	{
		// Mask the respective bits for the address (9 bits per level, starting at bit 12).
		const usize shift = (12 + (9 * (lvl - 1)));
		// Index into the current level map.
		index = (virt_val >> shift) & 0x1FF;

		if (lvl == 1 || cur_head[index] & PT_SIZE)
			break;

		// Update the head.
		cur_head = vm_x86_traverse(cur_head, index, false);
		if (cur_head == NULL)
		{
			spin_unlock(&page_map->lock);
			return false;
		}
	}

	if ((cur_head[index] & PT_PRESENT) == 0)
	{
		spin_unlock(&page_map->lock);
		return false;
	}

	// Clear everything.
	cur_head[index] = 0;
	vm_flush_tlb(virt_addr);
	spin_unlock(&page_map->lock);

	return true;
}

PhysAddr vm_virt_to_phys(PageMap* page_map, VirtAddr address)
{
	kassert(page_map != NULL, "page_map may not be null!");
	usize* pte = vm_x86_get_pte(page_map, address, false);

	// If the page is not present or the entry doesn't exist, we can't return a physical address.
	if (pte == NULL || (*pte & PT_PRESENT) == false)
	{
		return ~0UL;
	}

	return ((*pte) & PT_ADDR_MASK) + (address & 0xFFF);
}

bool vm_is_mapped(PageMap* page_map, VirtAddr address, VMProt prot)
{
	kassert(page_map != NULL, "page_map may not be null!");
	PhysAddr phys = vm_virt_to_phys(page_map, address);

	// Address is not mapped at all.
	if (phys == ~0UL)
		return false;

	return true;
}

// Converts protection flags to x86 page flags.
static usize vm_flags_to_x86(VMProt prot, VMFlags flags)
{
	usize x86_flags = PT_PRESENT;

	if (flags & VMFlags_User)
		x86_flags |= PT_USER_MODE;
	if (prot & VMProt_Write)
		x86_flags |= PT_READ_WRITE;
	if ((prot & VMProt_Execute) == 0)
		x86_flags |= PT_EXECUTE_DISABLE;

	return x86_flags;
}

bool vm_map(PageMap* page_map, PhysAddr phys_addr, VirtAddr virt_addr, VMProt prot, VMFlags flags, VMLevel level)
{
	kassert(page_map != NULL, "No page map was provided! Unable to map page 0x%p to 0x%p!", phys_addr, virt_addr);
	kassert(phys_addr % arch_page_size == 0, "Physical address is not page aligned! Value: 0x%p", phys_addr);

	if ((flags & VMFlags_User) && (virt_addr & 0xFFFF800000000000))
	{
		print_error("Attempted to map a user page in higher half of address space! (virt_addr = 0x%p)\n", virt_addr);
		return false;
	}

	spin_lock(&page_map->lock);

	usize x86_flags = vm_flags_to_x86(prot, flags);
	u64* cur_head = page_map->head;
	usize index = 0;

	for (usize lvl = 4; lvl >= 1; lvl--)
	{
		// Mask the respective bits for the address (9 bits per level, starting at bit 12).
		const usize shift = (12 + (9 * (lvl - 1)));
		// Index into the current level map.
		index = (virt_addr >> shift) & 0x1FF;

		// If we want to map a page for user mode access, we also have to map all previous levels for that.
		if (x86_flags & PT_USER_MODE)
			cur_head[index] |= PT_USER_MODE;

		if (lvl == level)
		{
			if (lvl > VMLevel_Medium)
				x86_flags |= PT_SIZE;
			break;
		}

		// Update the head.
		cur_head = vm_x86_traverse(cur_head, index, true);

		if (cur_head == NULL)
		{
			spin_unlock(&page_map->lock);
			return false;
		}
	}

	cur_head[index] = (phys_addr & PT_ADDR_MASK) | (x86_flags & ~(PT_ADDR_MASK));
	spin_unlock(&page_map->lock);

	// TODO: Move this to vm_map_range.
	MemoryMapping mapping = {
		.physical = phys_addr,
		.virtual = virt_addr,
		.num_pages = 1,
		.prot = prot,
		.flags = flags,
	};
	list_push(&page_map->maps, mapping);

	return true;
}

bool vm_protect(PageMap* page_map, VirtAddr virt_addr, VMProt prot, VMFlags flags)
{
	bool result = vm_x86_remap(page_map, virt_addr, vm_flags_to_x86(prot, flags));
	if (result == true)
		vm_flush_tlb(virt_addr);
	return result;
}

Context* interrupt_pf_handler(usize isr, Context* regs, void* data)
{
	// CR2 holds the address that was accessed.
	usize cr2;
	asm_get_register(cr2, cr2);

#if !defined(NDEBUG)
	print_log("vm: Page fault! Attempted to access 0x%p (ip: 0x%p)!\n", cr2, regs->rip);

	// Present
	if (BIT(regs->error, 0))
		print_log("vm: \t- Fault was a protection violation\n");
	else
		print_log("vm: \t- Page was not present\n");

	// Write
	if (BIT(regs->error, 1))
		print_log("vm: \t- Fault was caused by a write access\n");
	else
		print_log("vm: \t- Fault was caused by a read access\n");

	// User
	if (BIT(regs->error, 2))
		print_log("vm: \t- Fault was caused by the user\n");
	else
		print_log("vm: \t- Fault was caused by the kernel\n");

	// Instruction fetch
	if (BIT(regs->error, 4))
		print_log("vm: \t- Fault was caused by an instruction fetch\n");

	// Check if SMAP is blocking this access
	if (can_smap & !BIT(regs->rflags, 18) & !(regs->cs & CPL_USER))
		print_log("vm: \t- SMAP is enabled\n");
#endif

	// If the current protection level wasn't 3, that means the page fault was
	// caused by the supervisor! If this happens, we messed up big time!
	if (regs->cs == offsetof(Gdt, kernel_code))
	{
		// If we can't recover, abort.
		print_log("vm: Fatal page fault in kernel mode while trying to access 0x%p! (Error: 0x%zx, RIP: 0x%p)\n", cr2,
				  regs->error, regs->rip);
		ktrace(regs);
		kabort();
	}

	// TODO: Handle user page fault.
	if (arch_current_cpu())
	{
		Process* proc = arch_current_cpu()->thread->parent;

		// If nothing can make the process recover, we have to put it out of its misery.
		proc_kill(proc, true);
		regs = sch_reschedule(regs);
	}

	return regs;
}

usize vm_get_page_size(VMLevel level)
{
	switch (level)
	{
		case VMLevel_Small: return 4 * KiB;
		case VMLevel_Medium: return 2 * MiB;
		case VMLevel_Large: return 1 * GiB;
		default: return 0;
	}
}
