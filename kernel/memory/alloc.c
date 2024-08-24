// Kernel memory allocation implementation.

#include <menix/common.h>
#include <menix/memory/alloc.h>

#include <string.h>

#ifdef CONFIG_allocator_slab
#include <menix/memory/slab.h>

void alloc_init()
{
	slab_init();
}

void* kmalloc(usize bytes)
{
	return slab_alloc(bytes);
}

void* kzalloc(usize bytes)
{
	void* mem = slab_alloc(bytes);
	memset(mem, 0, bytes);
	return mem;
}

void* krealloc(void* old, usize new_bytes)
{
	return slab_realloc(old, new_bytes);
}

void kfree(void* memory)
{
	slab_free(memory);
}

#else
#error "No allocator selected! Enable any allocator_* option and rebuild."
#endif
