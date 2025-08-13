#ifndef _KERNEL_MEM_ALLOC_H
#define _KERNEL_MEM_ALLOC_H

#include <kernel/mem/types.h>
#include <stddef.h>

typedef enum kmalloc_flags {
    KMF_KERNEL = 1 << 0, // Memory is used for kernel objects.
    KMF_NOZERO = 1 << 1, // Don't zero out the allocated memory.
    KMF_MEM32 = 1 << 2,  // Allocated memory needs to fit inside 32 bits.
    KMF_MEM20 = 1 << 3,  // Allocated memory needs to fit inside 20 bits.
} kmf_t;

// Allocates a region of memory which can be smaller than the page size.
// Returns `nullptr` if the allocator cannot provide an allocation for the
// given `length` + `flags` combination.
// Always returns `nullptr` if `length` is 0.
void* kmalloc(size_t length, kmf_t flags, const char* name);

// Frees an allocation created by `kmalloc`.
// Passing `nullptr` is a no-op.
void kfree(void* mem);

#endif
