// Kernel memory allocation

#pragma once
#include <menix/common.h>

// All kalloc family functions assume that the physical and virtual memory management has been initialized
// prior to the first method call.

// Returns a memory region with at least `bytes` bytes of uninitialized memory.
void* kalloc(usize bytes);

// Returns a memory region with at least `bytes` bytes of uninitialized memory, aligned to `alignment`.
// `alignment` may not exceed `bytes`.
void* kaalloc(usize bytes, usize alignment);

// Returns a memory region with at least `bytes` bytes of zero-initialized memory.
void* kzalloc(usize bytes);

// Frees a block of memory allocated by one of the `k*alloc` functions.
// `memory` is a reference to the address.
// Any reference to the memory region is invalid after calling this.
void kfree(void** memory);
