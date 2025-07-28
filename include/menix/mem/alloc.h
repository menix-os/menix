#ifndef _MENIX_MEM_ALLOC_H
#define _MENIX_MEM_ALLOC_H

#include <menix/types.h>
#include <menix/mem_types.h>

typedef enum kmalloc_flags {
	// Allocate memory for the kernel.
	KMF_KERNEL = 1 << 0,
	// Zero out the allocated memory.
	KMF_ZEROED = 1 << 1,
	// Allocated memory needs to fit inside 32 bits.
	KMF_MEM32 = 1 << 2,
	// Allocated memory needs to fit inside 20 bits.
	KMF_MEM20 = 1 << 3,
} kmf_t;

// Allocates a region of memory smaller than the page size.
void* kmalloc(usize length, kmf_t flags);

// Frees an allocation created by `kmalloc`.
void kfree(void* mem);

phys_t kpage_alloc(usize pages);
void kpage_free(phys_t first_page, usize pages);

#endif
