// Virtual memory management

#pragma once

#include <menix/common.h>
#include <menix/fs/handle.h>
#include <menix/memory/pm.h>

#define MENIX_BITS_INCLUDE
#include <bits/vm.h>
#undef MENIX_BITS_INCLUDE

// Temporarily makes user mappings accessible.
// Usage: vm_user_access({ do_stuff(); });
#define vm_user_access(scope) \
	vm_show_user(); \
	scope; \
	vm_hide_user()

typedef enum
{
	VMFlags_Read = 1 << 0,
	VMFlags_Write = 1 << 1,
	VMFlags_Execute = 1 << 2,

	VMFlags_WriteCombine = 1 << 8,
	VMFlags_MapFixed = 1 << 9
} VMFlags;

// Defined in <bits/vm.h> since page maps might require processor specific information.
typedef struct PageMap PageMap;

// Initializes the virtual memory mapping with a bootloader-provided physical memory map.
// `phys_base`: A virtual address memory mapped to physical 0x0.
// `kernel_base`: A physical address pointing to the memory where the kernel has been loaded.
void vm_init(void* phys_base, PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries);

// Returns a reference to the kernel page map.
PageMap* vm_get_kernel_map();

// Updates the active page map.
void vm_set_page_map(PageMap* page_map);

// Creates a new page map.
PageMap* vm_page_map_new();

// Creates a new page map by forking an existing one.
PageMap* vm_page_map_fork(PageMap* source);

// Destroys a page map.
void vm_page_map_destroy(PageMap* map);

// Creates a new mapping for a region of memory.
// If `page_map` is equal to vm_get_kernel_map(), the returned value may be interpreted as a `void*`.
VirtAddr vm_map(PageMap* page_map, VirtAddr hint, usize length, VMFlags flags, Handle* fd, usize off);

// Changes the protection of an existing virtual address. Returns true if successful.
bool vm_protect(PageMap* page_map, VirtAddr virt_addr, usize length, VMFlags flags);

// Unmaps a virtual address. Does nothing if the address isn't mapped. Returns true if successful.
bool vm_unmap(PageMap* page_map, VirtAddr virt_addr, usize length);

// Translates a virtual address to a physical address. Returns 0 if not mapped.
// `page_map`: The page map of the process to look at.
// `address`: The virtual address to translate.
PhysAddr vm_virt_to_phys(PageMap* page_map, VirtAddr address);

// Maps memory from a different address space into the kernel address space with all permissions.
// `page_map`: The page map to load the mapping from.
// `foreign_addr`: The start of the region mapped in `page_map` to copy over.
// `num_pages`: The amount of pages to map.
void* vm_map_foreign(PageMap* page_map, VirtAddr foreign_addr, usize num_pages);

// Removes a mapping created by vm_map_to_kernel. Returns true if successful.
// `kernel_addr`: Address returned by vm_map_foreign.
// `num_pages`: Amount of pages in the original mapping.
bool vm_unmap_foreign(void* kernel_addr, usize num_pages);

// Checks if an address is mapped with the given flags.
bool vm_is_mapped(PageMap* page_map, VirtAddr address, VMFlags flags);

// Make user memory inaccessible to the kernel.
void vm_hide_user();

// Unhide user memory so that it's accessible to the kernel.
void vm_show_user();
