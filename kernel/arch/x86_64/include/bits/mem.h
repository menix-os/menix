#pragma once

#include <kernel/types.h>
#include <stddef.h>
#include <stdint.h>

typedef uint64_t pte_t;

static inline size_t arch_mem_page_bits() {
    return 12;
}

static inline size_t arch_mem_level_bits() {
    return 9;
}

static inline size_t arch_mem_num_levels() {
    return 4;
}

enum pte_flags;
enum cache_mode;
struct page_table;

void arch_pte_clear(pte_t* pte);
pte_t arch_pte_build(phys_t addr, enum pte_flags flags, enum cache_mode cache);
bool arch_pte_is_present(pte_t* pte);
bool arch_pte_is_dir(pte_t* pte);
phys_t arch_pte_address(pte_t* pte);
void arch_mem_pt_set(struct page_table* pt);
