#pragma once

#include <kernel/mem/types.h>
#include <stddef.h>

// Allocates a region of memory which can be smaller than the page size.
// Returns `nullptr` if the allocator cannot provide an allocation for the
// given `length` + `flags`. Always returns `nullptr` if `length` is 0.
void* mem_alloc(size_t length, enum alloc_flags flags);

// Frees an allocation created by `mem_alloc`.
// Passing `nullptr` is a no-op.
void mem_free(void* mem);

// Allocates an amount of contiguous physical pages.
phys_t mem_page_alloc(size_t num_pages);

// Frees an allocated region of physical pages.
void mem_page_free(phys_t start, size_t num_pages);
