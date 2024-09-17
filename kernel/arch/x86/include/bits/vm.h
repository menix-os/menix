// x86-specific virtual memory management.

#pragma once

#ifndef MENIX_BITS_INCLUDE
#error "Don't include bits headers directly!"
#endif

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/system/arch.h>
#include <menix/thread/spin.h>

typedef enum : usize
{
	PAGE_PRESENT = (1 << 0),
	PAGE_READ_WRITE = (1 << 1),
	PAGE_USER_MODE = (1 << 2),
	PAGE_WRITE_THROUGH = (1 << 3),
	PAGE_CACHE_DISABLE = (1 << 4),
	PAGE_ACCESSED = (1 << 5),
	PAGE_DIRTY = (1 << 6),
	PAGE_SIZE = (1 << 7),
	PAGE_GLOBAL = (1 << 8),
	PAGE_AVAILABLE = (1 << 9),
	PAGE_ATTRIBUTE_TABLE = (1 << 10),
	PAGE_EXECUTE_DISABLE = (1UL << 63)
} PageFlags;

#define PAGE_PROTECTION_KEY(key) ((key & 0xFUL) << 59)
#define PAGE_ADDR				 (0x0000FFFFFFFFF000UL)

typedef struct PageMap
{
	usize* head;
	SpinLock lock;
} PageMap;

// Maps a virtual address to physical memory. Returns true if successful.
bool vm_x86_map_page(PageMap* page_map, PhysAddr phys_addr, VirtAddr virt_addr, usize flags);

// Redefines an existing mapping. Returns true if successful.
bool vm_x86_remap_page(PageMap* page_map, VirtAddr virt_addr, usize flags);

// Destroys an existing mapping. Returns true if successful.
bool vm_x86_unmap_page(PageMap* page_map, VirtAddr virt_addr);

// Page fault interrupt handler. Set by vm_init().
void interrupt_pf_handler(CpuRegisters* regs);
