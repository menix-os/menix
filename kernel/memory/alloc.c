// Kernel memory allocation implementation.

#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/memory/pm.h>
#include <menix/memory/vm.h>

#include <string.h>

typedef enum
{
	AllocFlags_None = 0,
	AllocFlags_ForceAlignment = 1 << 0,
	AllocFlags_SetZero = 1 << 1,
} AllocFlags;

void alloc_init()
{
}

// Main allocation function.
// `bytes`: Minimum amount of bytes to allocate.
// `alignment`: Preferred alignment. If alignment is to be forced, pass ForceAlignment via flags.
// `flags`: Modfiy allocation behavior.
static void* allocate_inner(usize bytes, usize alignment, AllocFlags flags)
{
	// 0 alignment is illegal.
	if (alignment == 0)
		return NULL;

	// TODO: Right now we just give x amount of pages that meet the requirement.
	const usize pages = (bytes / CONFIG_page_size) + 1;
	void* result = pm_arch_alloc(pages) + pm_get_phys_base();

	// If requested, set memory to zero.
	if (flags & AllocFlags_SetZero)
		memset(result, 0, pages * CONFIG_page_size);

	return result;
}

void* kmalloc(usize bytes)
{
	void* mem = allocate_inner(bytes, 1, AllocFlags_None);
	return mem;
}

void* kaalloc(usize bytes, usize alignment)
{
	void* mem = allocate_inner(bytes, alignment, AllocFlags_ForceAlignment);
	return mem;
}

void* kcalloc(usize bytes)
{
	void* mem = allocate_inner(bytes, 1, AllocFlags_SetZero);
	return mem;
}

void* krealloc(void* old, usize new_bytes)
{
	// TODO actually do something.
	return old;
}

void kfree(void* memory)
{
	if (memory == NULL)
		return;

	// TODO: Free the region.
	PhysAddr base = memory - pm_get_phys_base();
	pm_arch_free(base);
}
