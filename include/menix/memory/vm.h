// Virtual memory management

#pragma once

#include <menix/common.h>
#include <menix/fs/handle.h>
#include <menix/memory/pm.h>

// Defined in <bits/vm.h> since page maps might require processor specific information.
typedef struct PageMap PageMap;

// Initializes the virtual memory mapping with a bootloader-provided physical memory map.
// `phys_base`: A virtual address memory mapped to physical 0x0.
// `kernel_base`: A physical address pointing to the memory where the kernel has been loaded.
void vm_init(void* phys_base, PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries);

// Returns a reference to the kernel page map.
PageMap* vm_get_kernel_map();

// Creates a new mapping for a region of memory.
void* vm_map(PageMap* page_map, VirtAddr hint, usize length, int prot, int flags, Handle* fd, usize off);

// Changes the protection of an existing virtual address. Returns true if successful.
bool vm_protect(PageMap* page_map, void* virt_addr, usize length, usize prot);

// Unmaps a virtual address. Does nothing if the address isn't mapped. Returns true if successful.
bool vm_unmap(PageMap* page_map, void* virt_addr, usize length);

// Translates a virtual address to a physical address. Returns 0 if not mapped.
// `page_map`: The page map of the process to look at.
// `address`: The virtual address to translate.
PhysAddr vm_virt_to_phys(PageMap* page_map, void* address);

#include <bits/vm.h>
