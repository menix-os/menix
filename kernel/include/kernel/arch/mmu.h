#ifndef _KERNEL_ARCH_MMU_H
#define _KERNEL_ARCH_MMU_H

#include <kernel/common.h>
#include <kernel/types.h>
#include <bits/mmu.h>
#include <stddef.h>

ASSERT_TYPE(pte_t);

enum pte_flags;
enum cache_mode;

// Clears out a page table entry.
void mmu_pte_clear(pte_t* pte);

// Builds a page table entry from the given info.
pte_t mmu_pte_build(phys_t addr, enum pte_flags flags, enum cache_mode cache);

// Gets the page size.
size_t mmu_page_size();

// The amount of bits in an address for lower half virtual memory.
size_t mmu_lower_half_bits();

#endif
