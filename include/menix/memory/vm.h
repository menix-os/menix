// Virtual memory management

#pragma once

#include <menix/common.h>
#include <menix/memory/pm.h>

typedef struct PageMap PageMap;

// Initializes the virtual memory mapping with a bootloader-provided physical memory map.
// `phys_base`: A virtual address memory mapped to physical 0x0.
// `kernel_base`: A physical address pointing to the memory where the kernel has been loaded.
void vm_init(void* phys_base, PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries);

// Creates a new mapping for a region of memory.
// `page_map`: The page map of the address space in which the mapping should be created.
// `hint`: A hint as to where to place the address. This value can't be forcefully used, as for example if the
void* vm_map(PageMap* page_map, void* hint, usize length, int prot, int flags);

// Unmaps a virtual address. Does nothing if the address isn't mapped.
// `page_map`: The page map of the address space from which the mapping should be removed.
void vm_unmap(PageMap* page_map, void* virt_addr);

#include <bits/vm.h>
