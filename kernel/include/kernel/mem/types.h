#pragma once

#include <kernel/compiler.h>
#include <kernel/types.h>
#include <stddef.h>

enum alloc_flags {
    AF_KERNEL = 1 << 0, // Memory is used for kernel objects.
    AF_NOZERO = 1 << 1, // Don't zero out the allocated memory.
    AF_MEM32 = 1 << 2,  // Allocated memory needs to fit inside 32 bits.
    AF_MEM20 = 1 << 3,  // Allocated memory needs to fit inside 20 bits.
};

enum phys_mem_usage {
    PHYS_RESERVED,
    PHYS_USABLE,
    PHYS_RECLAIMABLE,
};

struct phys_mem {
    phys_t address;
    size_t length;
    enum phys_mem_usage usage;
};

struct page {
    size_t flags;
    size_t refcount;
    union {
        struct {
            struct page* next;
            size_t length;
        } freelist;
    };
};
static_assert(0x1000 % sizeof(struct page) == 0, "struct must be a multiple of the page size!");
