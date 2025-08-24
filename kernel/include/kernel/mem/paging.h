#pragma once

#include <kernel/sys/spin.h>
#include <kernel/types.h>
#include <bits/mmu.h>

ASSERT_TYPE(pte_t);

enum pte_flags {
    PTE_READ = 1 << 0,  // Can read from this page.
    PTE_WRITE = 1 << 1, // Can write to this page.
    PTE_EXEC = 1 << 2,  // Can execute code on this page.
    PTE_USER = 1 << 3,  // Can be accessed by the user.
};

enum cache_mode {
    CACHE_NONE,
    CACHE_WRITE_COMBINE,
    CACHE_WRITE_THROUGH,
    CACHE_WRITE_BACK,
    CACHE_MMIO,
};

struct page_table {
    phys_t root;
    struct spinlock lock;
};

// Creates a new page table.
bool mem_pt_new(struct page_table* pt);

// Maps a single page to a virtual address in the given page table.
bool mem_pt_map(struct page_table* pt, virt_t vaddr, phys_t paddr, enum pte_flags flags);

// Changes the protection of a page.
bool mem_pt_protect(struct page_table* pt, virt_t vaddr, enum pte_flags flags);

// Unmaps a page.
bool mem_pt_unmap(struct page_table* pt, virt_t vaddr);

// Bit shift in a page.
static inline size_t mem_page_bits() {
    return arch_mem_page_bits();
}

// Gets the page size in bytes.
static inline size_t mem_page_size() {
    return 1 << mem_page_bits();
}

// The amount of bits in a level.
static inline size_t mem_level_bits() {
    return arch_mem_level_bits();
}

// The amount of levels in an address.
static inline size_t mem_num_levels() {
    return arch_mem_num_levels();
}

// Clears out a page table entry.
static inline void pte_clear(pte_t* pte) {
    return arch_pte_clear(pte);
}

// Builds a page table entry from the given info.
static inline pte_t pte_build(phys_t addr, enum pte_flags flags, enum cache_mode cache) {
    return arch_pte_build(addr, flags, cache);
}

// Returns true if the given PTE is present and valid.
static inline bool pte_is_present(pte_t* pte) {
    return arch_pte_is_present(pte);
}

// Returns true if the given PTE contains another level.
static inline bool pte_is_dir(pte_t* pte) {
    return arch_pte_is_dir(pte);
}

// Returns the address component of the page table entry.
static inline phys_t pte_address(pte_t* pte) {
    return arch_pte_address(pte);
}
