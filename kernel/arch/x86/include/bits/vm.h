// x86-specific virtual memory management.

#pragma once

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/thread/spin.h>

typedef struct
{
	usize* head;
	SpinLock lock;
} PageMap;

// Translates a virtual address to a physical address.
// Returns 0 if not mapped.
PhysAddr vm_arch_virt_to_phys(void* address);

// Maps a virtual address to physical memory. Returns true if successful.
bool vm_arch_map_page(PageMap* page_map, PhysAddr phys_addr, void* virt_addr, usize flags);

// Redefines an existing mapping. Returns true if successful.
bool vm_arch_remap_page(PageMap* page_map, PhysAddr phys_addr, void* virt_addr, usize flags);

// Unmaps a virtual address.
// Does nothing if the address isn't mapped.
void vm_arch_unmap_page(PageMap* page_map, void* virt_addr);
