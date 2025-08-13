#ifndef _KERNEL_MEM_PM_H
#define _KERNEL_MEM_PM_H

#include <kernel/mem/types.h>
#include <stddef.h>

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
static_assert(0x1000 % sizeof(struct page) == 0, "must be a multiple of the page size!");

extern struct page* page_db;

#endif
