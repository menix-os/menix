// Virtual memory management

#pragma once

#include <menix/common.h>
#include <menix/memory/pm.h>

// Initializes the virtual memory mapping with a bootloader-provided physical memory map.
void vm_init(PhysMemoryMap* mem_map);

// Translates a virtual address to a physical address.
// Returns 0 if not mapped.
PhysAddr vm_virt_to_phys(void* address);

// Maps a virtual address to physical memory.
void vm_map_page(PhysAddr phys_addr, void* virt_addr);

// Unmaps a virtual address.
// Does nothing if the address isn't mapped.
void vm_unmap_page(void* virt_addr);
