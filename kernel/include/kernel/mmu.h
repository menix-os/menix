#ifndef _KERNEL_MMU_H
#define _KERNEL_MMU_H

#include <kernel/arch/mmu.h>

enum pte_flags {
    PTE_READ = 1 << 0,
    PTE_WRITE = 1 << 1,
    PTE_EXEC = 1 << 2,
    PTE_USER = 1 << 3,
};

enum cache_mode {
    CACHE_NONE = 0,
    CACH
};

struct page_table {
    phys_t root;
};

#endif
