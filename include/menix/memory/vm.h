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
	VMLevel_Small = 1,
	VMLevel_Medium = 2,
	VMLevel_Large = 3,
} VMLevel;

typedef struct
{
	SpinLock lock;
#if defined(__x86_64__)
	usize* head;
#elif defined(__aarch64__)
	usize* head[2];
#elif defined(__riscv) && (__riscv_xlen == 64)
	usize* head;
#elif defined(__loongarch__) && (__loongarch_grlen == 64))
	usize* head[2];
#endif
} PageMap;

extern PageMap* vm_kernel_map;
extern VirtAddr kernel_map_base;

#define VM_USER_STACK_SIZE	 0x200000
#define VM_USER_STACK_BASE	 0x200000
#define VM_KERNEL_STACK_SIZE 0x200000
#define VM_USER_MAP_BASE	 0x00007F0000000000
#define VM_MAP_BASE			 0xFFFF900000000000
#define VM_MEMORY_BASE		 0xFFFFA00000000000
#define VM_MODULE_BASE		 0xFFFFB00000000000

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
PageMap* vm_page_map_new();

// Creates a new page map by forking an existing one.
PageMap* vm_page_map_fork(PageMap* source);

// Destroys a page map.
void vm_page_map_destroy(PageMap* map);

// Translates a virtual address to a physical address. Returns 0 if not mapped.
// `page_map`: The page map of the process to look at.
// `address`: The virtual address to translate. Does not have to be page-aligned.
PhysAddr vm_virt_to_phys(PageMap* page_map, VirtAddr address);

// Returns the size of a page entry at a given level.
usize vm_get_page_size(VMLevel level);

// Makes user memory accessible to the kernel.
void vm_user_show();

// Make user memory inaccessible to the kernel.
void vm_user_hide();

// Reads `num` bytes from user address `src` to kernel address `dst`. Returns actual bytes read.
usize vm_user_read(Process* proc, void* dst, VirtAddr src, usize num);

// Writes `num` bytes from kernel address `src` to user address `dst`. Returns actual bytes written.
usize vm_user_write(Process* proc, VirtAddr dst, void* src, usize num);

// Maps a phyiscal address with a certain length of bytes anywhere and returns a virtual address.
void* vm_map_memory(PhysAddr phys_addr, usize len, VMProt prot);
