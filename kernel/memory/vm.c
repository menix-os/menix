#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

#include <string.h>

PageMap* vm_kernel_map = NULL;					 // Page map used for the kernel.
VirtAddr kernel_map_base = VM_MAP_BASE;			 // Start of mappings allocated to the kernel.
VirtAddr kernel_memory_base = VM_MEMORY_BASE;	 // Start of (device, DMA) memory mappings.

SEGMENT_DECLARE_SYMBOLS(text)
SEGMENT_DECLARE_SYMBOLS(rodata)
SEGMENT_DECLARE_SYMBOLS(data)

void vm_init(PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries)
{
	kassert(num_entries > 0, "No memory map entries given!");

	// Create a memory map for the kernel.
	vm_kernel_map = vm_page_map_new();

	// Map all physical space.
	// Check for the highest usable physical memory address, so we know how much memory to map.
	usize highest = 0;
	for (usize i = 0; i < num_entries; i++)
	{
		const usize region_end = mem_map[i].address + mem_map[i].length;
		if (region_end > highest)
			highest = region_end;
	}

	const void* phys_base = pm_get_phys_base();
	for (usize cur = 0; cur < highest; cur += vm_get_page_size(VMLevel_Large))
		kassert(vm_map(vm_kernel_map, cur, (VirtAddr)phys_base + cur, VMProt_Read | VMProt_Write, 0, VMLevel_Large),
				"Unable to map physical memory!");

	// Map the kernel segments to the current physical address again.
	for (usize cur = (usize)SEGMENT_START(text); cur < (usize)SEGMENT_END(text); cur += vm_get_page_size(VMLevel_Small))
		kassert(vm_map(vm_kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, cur, VMProt_Read | VMProt_Execute, 0,
					   VMLevel_Small),
				"Unable to map text segment!");

	for (usize cur = (usize)SEGMENT_START(rodata); cur < (usize)SEGMENT_END(rodata);
		 cur += vm_get_page_size(VMLevel_Small))
		kassert(vm_map(vm_kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, cur, VMProt_Read, 0, VMLevel_Small),
				"Unable to map rodata segment!");

	for (usize cur = (usize)SEGMENT_START(data); cur < (usize)SEGMENT_END(data); cur += vm_get_page_size(VMLevel_Small))
		kassert(vm_map(vm_kernel_map, cur - (PhysAddr)KERNEL_START + kernel_base, cur, VMProt_Read | VMProt_Write, 0,
					   VMLevel_Small),
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

void* vm_map_memory(PhysAddr phys_addr, usize len, VMProt prot)
{
	const usize page_size = vm_get_page_size(VMLevel_Small);
	const usize aligned_bytes = ALIGN_UP(len, page_size);
	const usize num_pages = aligned_bytes / page_size;
	phys_addr = ALIGN_DOWN(phys_addr, page_size);

	VirtAddr start = kernel_memory_base;
	for (usize page = 0; page < num_pages; page++)
	{
		vm_map(vm_kernel_map, phys_addr + (page_size * page), start + (page_size * page), prot, 0, VMLevel_Small);
	}
	spin_lock_scope(&vm_kernel_map->lock, { kernel_memory_base += aligned_bytes; });

	return (void*)start;
}

void* vm_map_foreign(PageMap* page_map, VirtAddr foreign_addr, usize num_pages)
{
	VirtAddr start = kernel_map_base;
	const usize page_size = vm_get_page_size(VMLevel_Small);

	for (usize page = 0; page < num_pages; page++)
	{
		// Physical page where the data lives.
		const PhysAddr foreign_phys = vm_virt_to_phys(page_map, foreign_addr + (page * page_size));
		// Virtual address in the kernel page map.
		const VirtAddr domestic_virt = start + (page * page_size);

		kassert(foreign_phys != ~0UL, "Unable to map to address 0x%p, because it isn't mapped in the target process!",
				foreign_addr);

		if (vm_map(vm_kernel_map, foreign_phys, domestic_virt, VMProt_Read | VMProt_Write, 0, VMLevel_Small) == false)
		{
			return (void*)~0UL;
		}
	}

	// TODO: This is really bad and might cause a crash if left running for a really long time.
	// It's a better idea to keep track of these maps, just like the PM.
	kernel_map_base += num_pages * page_size;

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
