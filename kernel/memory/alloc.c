// Kernel memory allocation implementation.

#include <menix/common.h>
#include <menix/memory/alloc.h>

#include <string.h>

typedef enum
{
	AllocFlags_None = 0,
	AllocFlags_IgnoreAlignment = 1 << 0,
	AllocFlags_SetZero = 1 << 1,
} AllocFlags;

// Main allocation function.
// `bytes`: Minimum amount of bytes to allocate.
// `alignment`: Preferred alignment. If alignment is to be ignored, pass IgnoreAlignment via flags.
// `flags`: Modfiy allocation behavior.
static void* allocate_inner(usize bytes, usize alignment, AllocFlags flags)
{
	// TODO
	return NULL;
}

void* kalloc(usize bytes)
{
	void* mem = allocate_inner(bytes, 1, AllocFlags_IgnoreAlignment);
	return mem;
}

void* kaalloc(usize bytes, usize alignment)
{
	void* mem = allocate_inner(bytes, alignment, AllocFlags_None);
	return mem;
}

void* kzalloc(usize bytes)
{
	void* mem = allocate_inner(bytes, 1, AllocFlags_IgnoreAlignment | AllocFlags_SetZero);
	return mem;
}
