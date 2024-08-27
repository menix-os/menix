// x86-specific virtual memory management.

#pragma once

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/thread/spin.h>

#define PAGE_PRESENT			 (1 << 0)
#define PAGE_READ_WRITE			 (1 << 1)
#define PAGE_USER_MODE			 (1 << 2)
#define PAGE_WRITE_THROUGH		 (1 << 3)
#define PAGE_CACHE_DISABLE		 (1 << 4)
#define PAGE_ACCESSED			 (1 << 5)
#define PAGE_DIRTY				 (1 << 6)
#define PAGE_SIZE				 (1 << 7)
#define PAGE_GLOBAL				 (1 << 8)
#define PAGE_AVAILABLE			 (1 << 9)
#define PAGE_ATTRIBUTE_TABLE	 (1 << 10)
#define PAGE_PROTECTION_KEY(key) ((key & 0xFUL) << 59)
#define PAGE_EXECUTE_DISABLE	 (1ULL << 63)
#define PAGE_ADDR				 (0x0000FFFFFFFFF000UL)

typedef struct PageMap
{
	usize* head;
	SpinLock lock;
} PageMap;

// Updates the active page map.
void vm_arch_set_page_map(PageMap* map);

// Translates a virtual address to a physical address.
// Returns 0 if not mapped.
PhysAddr vm_arch_virt_to_phys(PageMap* page_map, void* address);

// Maps a virtual address to physical memory. Returns true if successful.
bool vm_arch_map_page(PageMap* page_map, PhysAddr phys_addr, void* virt_addr, usize flags);

// Redefines an existing mapping. Returns true if successful.
bool vm_arch_remap_page(PageMap* page_map, PhysAddr phys_addr, void* virt_addr, usize flags);

// Page fault interrupt handler. Set by vm_init().
void vm_page_fault_handler(CpuRegisters* regs);
