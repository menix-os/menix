#ifndef _KERNEL_MEM_H
#define _KERNEL_MEM_H

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

extern struct page* mem_page_db;

// Initializes all memory management structures.
void mem_init(struct phys_mem* map, size_t length, virt_t kernel_virt, phys_t kernel_phys, virt_t hhdm_address);

// Allocates a region of memory which can be smaller than the page size.
// Returns `nullptr` if the allocator cannot provide an allocation for the
// given `length` + `flags` combination. Always returns `nullptr` if `length` is 0.
void* mem_alloc(size_t length, enum alloc_flags flags, const char* name);

// Frees an allocation created by `mem_alloc`.
// Passing `nullptr` is a no-op.
void mem_free(void* mem);

// Allocate a single page of physical memory.
phys_t mem_page_alloc();

// Copies a block of data from user to kernel memory.
void copy_from_user(uint8_t* dst, const uint8_t __user* src, size_t num);

// Copies a block of data from kernel to user memory.
void copy_to_user(uint8_t __user* dst, const uint8_t* src, size_t num);

#endif
