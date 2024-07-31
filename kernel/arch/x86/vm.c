// Virtual memory management for x86.

#include <menix/memory/vm.h>

#define vm_flush_tlb(addr) asm volatile("invlpg (%0)" ::"r"(addr) : "memory")

void vm_init(PhysMemoryMap* mem_map)
{
}

PhysAddr vm_virt_to_phys(void* address)
{
	return 0;
}

void vm_map_page(PhysAddr phys_addr, void* virt_addr)
{
}

void vm_unmap_page(void* virt_addr)
{
}
