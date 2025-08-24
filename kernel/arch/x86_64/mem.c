#include <kernel/mem/paging.h>
#include <kernel/types.h>

enum : pte_t {
    ARCH_FLAG_NONE = 0,
    ARCH_FLAG_PRESENT = 1 << 0,
    ARCH_FLAG_READ_WRITE = 1 << 1,
    ARCH_FLAG_USER_MODE = 1 << 2,
    ARCH_FLAG_WRITE_THROUGH = 1 << 3,
    ARCH_FLAG_CACHE_DISABLE = 1 << 4,
    ARCH_FLAG_ACCESSED = 1 << 5,
    ARCH_FLAG_DIRTY = 1 << 6,
    ARCH_FLAG_SIZE = 1 << 7,
    ARCH_FLAG_GLOBAL = 1 << 8,
    ARCH_FLAG_AVAILABLE = 1 << 9,
    ARCH_FLAG_ATTRIBUTE_TABLE = 1 << 10,
    ARCH_FLAG_EXECUTE_DISABLE = 1LU << 63,

    ARCH_PTE_ADDR_MASK = 0x000F'FFFF'FFFF'F000,
};

void arch_pte_clear(pte_t* pte) {
    *pte = 0;
}

pte_t arch_pte_build(phys_t addr, enum pte_flags flags, enum cache_mode cache) {
    pte_t result = ((pte_t)addr & ARCH_PTE_ADDR_MASK);

    if (flags & PTE_WRITE)
        result |= ARCH_FLAG_READ_WRITE;
    if (!(flags & PTE_EXEC))
        result |= ARCH_FLAG_EXECUTE_DISABLE;

    return result;
}

bool arch_pte_is_present(pte_t* pte) {
    return *pte & ARCH_FLAG_PRESENT;
}

bool arch_pte_is_dir(pte_t* pte) {
    return *pte & ARCH_FLAG_SIZE;
}

phys_t arch_pte_address(pte_t* pte) {
    return (phys_t)(*pte & ARCH_PTE_ADDR_MASK);
}
