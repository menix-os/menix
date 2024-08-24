// Kernel memory allocation

#pragma once
#include <menix/common.h>

// All kalloc family functions assume that the physical and virtual memory management has been initialized
// prior to the first method call.
void alloc_init();

// Returns a memory region with at least `bytes` bytes of uninitialized memory.
void* kmalloc(usize bytes);

// Returns a memory region with at least `bytes` bytes of zero-initialized memory.
void* kzalloc(usize bytes);

// Reallocates a memory region with at least `new_bytes` bytes of uninitialized memory.
// If a new memory region has to be allocated to fit the request, the old data is copied over.
void* krealloc(void* old, usize new_bytes);

// Frees a block of memory allocated by one of the `k*alloc` functions.
// `memory`: The address to free.
// Any reference to the memory region is invalid after calling this.
void kfree(void* memory);
