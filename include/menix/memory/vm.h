// Virtual memory management

#pragma once

#include <menix/common.h>
#include <menix/fs/handle.h>
#include <menix/memory/pm.h>

#define MENIX_BITS_INCLUDE
#include <bits/vm.h>
#undef MENIX_BITS_INCLUDE

typedef enum
{
	VMProt_Read = 1 << 0,
	VMProt_Write = 1 << 1,
	VMProt_Execute = 1 << 2,
} VMProt;

typedef enum
{
	VMFlags_WriteCombine = 1 << 0,
} VMFlags;

// Defined in <bits/vm.h> since page maps might require processor specific information.
typedef struct PageMap PageMap;

// Initializes the virtual memory mapping with a bootloader-provided physical memory map.
// `kernel_base`: A physical address pointing to the memory where the kernel has been loaded.
void vm_init(PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries);

// Creates a new mapping for a region of memory.
bool vm_map(PageMap* page_map, PhysAddr phys_addr, VirtAddr virt_addr, VMProt prot, VMFlags flags);

// Changes the protection of an existing virtual address. Returns true if successful.
bool vm_protect(PageMap* page_map, VirtAddr virt_addr, VMProt prot);

// Unmaps a virtual address. Does nothing if the address isn't mapped. Returns true if successful.
bool vm_unmap(PageMap* page_map, VirtAddr virt_addr);

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
bool vm_is_mapped(PageMap* page_map, VirtAddr address, VMProt prot);

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

// Translates a virtual address to a physical address. Returns 0 if not mapped.
// `page_map`: The page map of the process to look at.
// `address`: The virtual address to translate.
PhysAddr vm_virt_to_phys(PageMap* page_map, VirtAddr address);
