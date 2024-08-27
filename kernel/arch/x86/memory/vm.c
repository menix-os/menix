// Virtual memory management for x86.

#include "bits/vm.h"

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/memory/alloc.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/util/self.h>

#include <string.h>

#define vm_flush_tlb(addr) asm volatile("invlpg (%0)" ::"r"(addr) : "memory")

SEGMENT_DECLARE_SYMBOLS(text)
SEGMENT_DECLARE_SYMBOLS(rodata)
SEGMENT_DECLARE_SYMBOLS(data)

PageMap* kernel_map = NULL;		  // Page map used for the kernel.
static void* phys_addr = NULL;	  // Memory mapped lower physical memory.

void vm_init(void* phys_base, PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries)
{
	phys_addr = phys_base;
	kassert(num_entries > 0, "No memory map entries given!");

	// Get a pointer to the first free physical memory page. Here we'll allocate our page directory structure.
	kernel_map = phys_addr + pm_arch_alloc(1);
	kernel_map->lock = spin_new();
	kernel_map->head = phys_addr + pm_arch_alloc(1);
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
		kassert(vm_arch_map_page(kernel_map, cur, phys_addr + cur, PAGE_PRESENT | PAGE_READ_WRITE | PAGE_SIZE),
				"Unable to map lower memory!\n");

	// Map the kernel segments to the current physical address again.
	for (usize cur = (usize)SEGMENT_START(text); cur < (usize)SEGMENT_END(text); cur += CONFIG_page_size)
		kassert(vm_arch_map_page(kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, (void*)cur, PAGE_PRESENT),
				"Unable to map text segment!\n");

	for (usize cur = (usize)SEGMENT_START(rodata); cur < (usize)SEGMENT_END(rodata); cur += CONFIG_page_size)
		kassert(vm_arch_map_page(kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, (void*)cur,
								 PAGE_PRESENT | PAGE_EXECUTE_DISABLE),
				"Unable to map rodata segment!\n");

	for (usize cur = (usize)SEGMENT_START(data); cur < (usize)SEGMENT_END(data); cur += CONFIG_page_size)
		kassert(vm_arch_map_page(kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, (void*)cur,
								 PAGE_PRESENT | PAGE_READ_WRITE | PAGE_EXECUTE_DISABLE),
				"Unable to map data segment!\n");

	// If the physical base ever changes, update it in the physical memory manager as well.
	pm_update_phys_base(phys_addr);

	// Load the new page directory.
	vm_arch_set_page_map(kernel_map);
}

void vm_arch_set_page_map(PageMap* map)
{
	asm_set_register(((usize)map->head - (usize)phys_addr), cr3);
}

PhysAddr vm_arch_virt_to_phys(PageMap* page_map, void* address)
{
	// TODO
	return 0;
}

// Returns the next level of the current page map level. Optionally allocates a page.
static u64* vm_arch_traverse(u64* top, usize idx, bool allocate)
{
	// If we have allocated the next level, filter the address as offset and return the level.
	if (top[idx] & PAGE_PRESENT)
		return (u64*)(phys_addr + (top[idx] & PAGE_ADDR));

	// If we don't want to allocate a page, but there was no page present, we can't do anything here.
	if (!allocate)
		return NULL;

	// Allocate the next level (will contain `CONFIG_page_size/sizeof(u64) == 512` entries).
	PhysAddr next_level = pm_arch_alloc(1);
	memset(phys_addr + next_level, 0x00, CONFIG_page_size);

	// Mark the next level as present so we don't allocate again.
	top[idx] = (u64)next_level | PAGE_PRESENT | PAGE_READ_WRITE;

	return (u64*)(phys_addr + next_level);
}

bool vm_arch_map_page(PageMap* page_map, PhysAddr phys_addr, void* virt_addr, usize flags)
{
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

		// If we allocate a 2MiB page, there is one less level in that page map branch.
		// In either case, don't traverse further after setting the index for writing.
		if (lvl == (flags & PAGE_SIZE ? 2 : 1))
			break;

		// Update the head.
		cur_head = vm_arch_traverse(cur_head, index, true);
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

void* vm_map(PageMap* page_map, void* hint, usize length, int prot, int flags)
{
	usize x86_flags = PAGE_EXECUTE_DISABLE;
	// TODO: Convert flags to x86 page flags.
	if (page_map != kernel_map)
		x86_flags |= PAGE_USER_MODE;

	void* addr = NULL;

	// Check the hint and make changes if necessary.
	if (hint != NULL && (usize)hint >= CONFIG_vm_mmap_min_addr)
	{
		// Make sure the page is boundary-aligned.
		// Do this by first checking if there already is a mapping at the hinted address.
		hint = (void*)ALIGN_DOWN((usize)hint, CONFIG_page_size);
		// If there is not, we can respect the hint.
	}
	// Choose a free region of program memory if no hint was given.
	else
	{
	}

	const usize page_count = 1 + (length / CONFIG_page_size);
	// Allocate individual pages.
	for (usize i = 0; i < page_count; i++)
	{
		PhysAddr page = pm_arch_alloc(page_count);
		vm_arch_map_page(page_map, page, addr + (i * CONFIG_page_size), x86_flags);
	}

	return addr;
}

void vm_unmap(PageMap* page_map, void* virt_addr)
{
	// TODO
	vm_flush_tlb(virt_addr);
}

void vm_page_fault_handler(CpuRegisters* regs)
{
	usize cr2;
	asm_get_register(cr2, cr2);

	// If the current protection level wasn't 3, that means the page fault was
	// caused by the supervisor! If this happens, we messed up big time!
	if (regs->cs & CPL_USER)
	{
		// TODO Handle user page fault.
	}
	else
	{
		kmesg("Page fault in supervisor mode!\n");
		kmesg("cs: 0x%hx cr2: 0x%p\n", regs->cs, cr2);
		kmesg("Error: 0x%zu\n", regs->error);
		kabort();
	}
}
