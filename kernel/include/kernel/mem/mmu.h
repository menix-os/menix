#ifndef _KERNEL_MEM_MMU_H
#define _KERNEL_MEM_MMU_H

#include <kernel/mem/types.h>
#include <kernel/util/compiler.h>
#include <bits/mem/mmu.h>

// Gets the physical address from the PTE.
static inline phys_t pte_get_addr(pte_t pte);

// Returns true if the PTE is marked present.
static inline bool pte_is_present(pte_t pte);

#endif
