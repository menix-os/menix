// x86-specific virtual memory management.

#pragma once

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/thread/spin.h>

typedef struct PageMap
{
	usize* head;
	SpinLock lock;
} PageMap;

typedef enum
{
	PageSize_4KiB,
	PageSize_2MiB,
} PageSize;

// Updates the active page map.
void vm_arch_set_page_map(PageMap* map);

// Translates a virtual address to a physical address.
// Returns 0 if not mapped.
PhysAddr vm_arch_virt_to_phys(PageMap* page_map, void* address);

// Maps a virtual address to physical memory. Returns true if successful.
bool vm_arch_map_page(PageMap* page_map, PhysAddr phys_addr, void* virt_addr, usize flags, PageSize size);

// Redefines an existing mapping. Returns true if successful.
bool vm_arch_remap_page(PageMap* page_map, PhysAddr phys_addr, void* virt_addr, usize flags);

// Unmaps a virtual address.
// Does nothing if the address isn't mapped.
void vm_arch_unmap_page(PageMap* page_map, void* virt_addr);

// Page fault interrupt handler. Set by vm_init().
void vm_page_fault_handler(u32 fault, u32 code);
