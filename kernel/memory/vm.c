#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/util/log.h>

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
	vm_kernel_map->lock = (SpinLock) {0};

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
}

usize vm_user_read(Process* proc, void* dst, VirtAddr src, usize num)
{
	if (proc == NULL || dst == NULL || num == 0)
		return 0;

	usize written = 0;

	// TODO: Check if the memory is mapped and copy the buffer page wise.
	vm_user_show();
	memcpy(dst, (void*)src, num);
	written += num;
	vm_user_hide();

	return written;
}

usize vm_user_write(Process* proc, VirtAddr dst, void* src, usize num)
{
	if (proc == NULL || src == NULL || num == 0)
		return 0;

	usize written = 0;

	// TODO: Check if the memory is mapped and copy the buffer page wise.
	vm_user_show();
	memcpy((void*)dst, src, num);
	written += num;
	vm_user_hide();

	return written;
}

void* vm_map_foreign(PageMap* page_map, VirtAddr foreign_addr, usize num_pages)
{
	VirtAddr start = vm_kernel_foreign_base;

	for (usize page = 0; page < num_pages; page++)
	{
		// Physical page where the data lives.
		const PhysAddr foreign_phys = vm_virt_to_phys(page_map, foreign_addr + (page * vm_get_page_size(VMLevel_0)));
		// Virtual address in the kernel page map.
		const VirtAddr domestic_virt = start + (page * vm_get_page_size(VMLevel_0));

		kassert(foreign_phys != ~0, "Unable to map to an address that isn't mapped in the target process!");

		if (vm_map(vm_kernel_map, foreign_phys, domestic_virt, VMProt_Read | VMProt_Write, 0, VMLevel_0) == false)
		{
			return (void*)~0UL;
		}
	}

	// TODO: This is really bad and might cause a crash if left running for a really long time.
	// It's a better idea to keep track of these maps, just like the PM.
	vm_kernel_foreign_base += num_pages * vm_get_page_size(VMLevel_0);

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
