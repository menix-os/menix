// Virtual memory management for x86.

#include <menix/common.h>
#include <menix/log.h>
#include <menix/memory/vm.h>

#include "menix/memory/pm.h"

#define PAGE_PRESENT			 (1 << 0)
#define PAGE_READ_WRITE			 (1 << 1)
#define PAGE_USER_MODE			 (1 << 2)
#define PAGE_WRITE_THROUGH		 (1 << 3)
#define PAGE_CACHE_DISABLE		 (1 << 4)
#define PAGE_ACCESSED			 (1 << 5)
#define PAGE_DIRTY				 (1 << 6)
#define PAGE_SIZE				 (1 << 7)
#define PAGE_GLOBAL				 (1 << 8)
#define PAGE_AVAILABLE			 (1 << 9)
#define PAGE_ATTRIBUTE_TABLE	 (1 << 10)
#define PAGE_PROTECTION_KEY(key) ((key & 0xF) << 59)
#define PAGE_EXECUTE_DISABLE	 (1 << 63)

#define vm_flush_tlb(addr) asm volatile("invlpg (%0)" ::"r"(addr) : "memory")

typedef struct
{
} PageMap;

void vm_init(void* phys_base, PhysMemory* mem_map, usize num_entries)
{
	// TODO
	kassert(num_entries >= 1, "No memory map entries given!");

	// Get a pointer to the first free physical memory page. Here we'll allocate our page directory structure.
	usize kernel;
	for (kernel = 0; kernel < num_entries; kernel++)
	{
		if (mem_map[kernel].usage == PhysMemoryUsage_Free)
			break;
	}

	// Map ourselves.

	// Load the new page directory.
	// asm volatile("mov %0, %%cr3" ::"r"(mem_map[0].address) : "memory");
}

PhysAddr vm_virt_to_phys(void* address)
{
	return 0;
}

void vm_map_page(PhysAddr phys_addr, void* virt_addr)
{
	vm_flush_tlb(virt_addr);
}

void vm_unmap_page(void* virt_addr)
{
	vm_flush_tlb(virt_addr);
}
