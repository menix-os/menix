#include <menix/memory/vm.h>
#include <menix/system/arch.h>

#include <string.h>

PageMap* vm_kernel_map = NULL;									 // Page map used for the kernel.
VirtAddr vm_kernel_foreign_base = CONFIG_vm_map_foreign_base;	 // Start of foreign mappings.

SEGMENT_DECLARE_SYMBOLS(text)
SEGMENT_DECLARE_SYMBOLS(rodata)
SEGMENT_DECLARE_SYMBOLS(data)

void vm_init(PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries)
{
	kassert(num_entries > 0, "No memory map entries given!");

	// Get a pointer to the first free physical memory page. Here we'll allocate our page directory structure.
	vm_kernel_map = pm_get_phys_base() + pm_alloc(1);
	vm_kernel_map->lock = spin_new();

#if defined(CONFIG_arch_x86_64)
	vm_kernel_map->head = pm_get_phys_base() + pm_alloc(1);
	memset(vm_kernel_map->head, 0x00, arch_page_size);
#elif defined(CONFIG_arch_riscv64)

#endif

	// Map all physical space.
	// Check for the highest usable physical memory address, so we know how much memory to map.
	usize highest = 0;
	for (usize i = 0; i < num_entries; i++)
	{
		const usize region_end = mem_map[i].address + mem_map[i].length;
		if (region_end > highest)
			highest = region_end;
	}

	const void* pyhs_base = pm_get_phys_base();
	for (usize cur = 0; cur < highest; cur += vm_get_page_size(VMLevel_2))
		kassert(vm_map(vm_kernel_map, cur, (VirtAddr)pyhs_base + cur, VMProt_Read | VMProt_Write, 0, VMLevel_2),
				"Unable to map physical memory!");

	// Map the kernel segments to the current physical address again.
	for (usize cur = (usize)SEGMENT_START(text); cur < (usize)SEGMENT_END(text); cur += vm_get_page_size(VMLevel_0))
		kassert(vm_map(vm_kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, cur, VMProt_Read | VMProt_Execute, 0,
					   VMLevel_0),
				"Unable to map text segment!");

	for (usize cur = (usize)SEGMENT_START(rodata); cur < (usize)SEGMENT_END(rodata); cur += vm_get_page_size(VMLevel_0))
		kassert(vm_map(vm_kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, cur, VMProt_Read, 0, VMLevel_0),
				"Unable to map rodata segment!");

	for (usize cur = (usize)SEGMENT_START(data); cur < (usize)SEGMENT_END(data); cur += vm_get_page_size(VMLevel_0))
		kassert(vm_map(vm_kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, cur, VMProt_Read | VMProt_Write, 0,
					   VMLevel_0),
				"Unable to map data segment!");

	// Load the new page directory.
	vm_set_page_map(vm_kernel_map);
	kmesg("Initialized virtual memory management!\n");
	kmesg("Page size: 0x%zx\n", vm_get_page_size(VMLevel_0));
}

void* vm_map_foreign(PageMap* page_map, VirtAddr foreign_addr, usize num_pages)
{
	VirtAddr start = vm_kernel_foreign_base;

	for (usize page = 0; page < num_pages; page++)
	{
		const PhysAddr foreign_phys = vm_virt_to_phys(page_map, foreign_addr + (page * vm_get_page_size(VMLevel_0)));
		const VirtAddr domestic_virt = start + (page * vm_get_page_size(VMLevel_0));

		if (vm_map(vm_kernel_map, foreign_phys, domestic_virt, VMProt_Read | VMProt_Write, 0, VMLevel_0) == false)
			return (void*)~0UL;
	}

	vm_kernel_foreign_base += num_pages * arch_page_size;

	return (void*)start;
}

bool vm_unmap_foreign(void* kernel_addr, usize num_pages)
{
	for (usize page = 0; page < num_pages; page++)
	{
		if (vm_unmap(vm_kernel_map, (VirtAddr)kernel_addr + (page * arch_page_size)) == false)
			return false;
	}
	return true;
}
