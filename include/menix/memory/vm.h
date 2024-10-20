// Virtual memory management

#pragma once

#include <menix/common.h>
#include <menix/fs/handle.h>
#include <menix/memory/pm.h>
#include <menix/util/self.h>

typedef enum
{
	VMProt_Read = 1 << 0,
	VMProt_Write = 1 << 1,
	VMProt_Execute = 1 << 2,
} VMProt;

typedef enum
{
	VMFlags_User = 1 << 0,
} VMFlags;

typedef enum : usize
{
#if defined(CONFIG_arch_x86_64) || defined(CONFIG_arch_aarch64) || defined(CONFIG_arch_riscv64) || \
	defined(CONFIG_loongarch64)
	VMLevel_0 = 1,
	VMLevel_1 = 2,
	VMLevel_2 = 3,
#endif
#if defined(CONFIG_arch_riscv64)
	VMLevel_4 = 4,
	VMLevel_5 = 5,
#endif
} VMLevel;

typedef struct
{
	SpinLock lock;
	VMLevel size;

#if defined(CONFIG_arch_x86_64)
	usize* head;
#elif defined(CONFIG_arch_aarch64)
	usize* head[2];
#elif defined(CONFIG_arch_riscv64)
	usize* head;
#elif defined(CONFIG_arch_loongarch64)
	usize* head[2];
#endif
} PageMap;

extern PageMap* vm_kernel_map;
extern VirtAddr vm_kernel_foreign_base;

// Initializes the virtual memory mapping with a bootloader-provided physical memory map.
// `kernel_base`: A physical address pointing to the memory where the kernel has been loaded.
void vm_init(PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries);

// Creates a new mapping for a region of memory.
bool vm_map(PageMap* page_map, PhysAddr phys_addr, VirtAddr virt_addr, VMProt prot, VMFlags flags, VMLevel size);

// Changes the protection of an existing virtual address. Returns true if successful.
bool vm_protect(PageMap* page_map, VirtAddr virt_addr, VMProt prot, VMFlags flags);

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

// Updates the active page map.
void vm_set_page_map(PageMap* page_map);

// Creates a new page map.
PageMap* vm_page_map_new(VMLevel size);

// Creates a new page map by forking an existing one.
PageMap* vm_page_map_fork(PageMap* source);

// Destroys a page map.
void vm_page_map_destroy(PageMap* map);

// Translates a virtual address to a physical address. Returns 0 if not mapped.
// `page_map`: The page map of the process to look at.
// `address`: The virtual address to translate.
PhysAddr vm_virt_to_phys(PageMap* page_map, VirtAddr address);

// Returns the size of a page entry at a given level.
usize vm_get_page_size(VMLevel level);
