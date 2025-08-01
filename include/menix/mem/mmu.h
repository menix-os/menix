#ifndef _MENIX_MEM_MMU_H
#define _MENIX_MEM_MMU_H

#include <menix/mem/types.h>
#include <menix/util/attributes.h>
#include <bits/mem/mmu.h>

// Gets the physical address from the PTE.
[[__arch]]
static inline phys_t pte_get_addr(pte_t pte);

// Returns true if the PTE is marked present.
[[__arch]]
static inline bool pte_is_present(pte_t pte);

#endif
