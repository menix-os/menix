// SLAB memory allocation

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/memory/slab.h>

#include <string.h>

static Slab slabs[8] = {0};

static void slab_new(Slab* slab, usize size)
{
	slab->lock = spin_new();
	// Allocate a new page for the head.
	slab->head = (void**)((usize)pm_alloc(1) + pm_get_phys_base());
	slab->ent_size = size;

	const usize offset = ALIGN_UP(sizeof(SlabHeader), size);
	const usize available_size = CONFIG_page_size - offset;

	SlabHeader* ptr = (SlabHeader*)slab->head;
	ptr->slab = slab;
	slab->head = (void**)((void*)slab->head + offset);

	void** arr = slab->head;
	const usize max = available_size / size - 1;
	const usize fact = size / sizeof(void*);

	for (usize i = 0; i < max; i++)
	{
		arr[i * fact] = &arr[(i + 1) * fact];
	}
	arr[max * fact] = NULL;
}

void slab_init(void)
{
	// Create slabs for common structure sizes to minimize overhead.
	slab_new(&slabs[0], 8);
	slab_new(&slabs[1], 16);
	slab_new(&slabs[2], 32);
	slab_new(&slabs[3], 64);
	slab_new(&slabs[4], 128);
	slab_new(&slabs[5], 256);
	slab_new(&slabs[6], 512);
	slab_new(&slabs[7], 1024);
}

static void* slab_do_alloc(Slab* slab)
{
	spin_acquire_force(&slab->lock);

	if (slab->head == NULL)
		slab_new(slab, slab->ent_size);

	void** old_free = slab->head;
	slab->head = *old_free;
	memset(old_free, 0, slab->ent_size);

	spin_free(&slab->lock);
	return old_free;
}

static void slab_do_free(Slab* slab, void* addr)
{
	spin_acquire_force(&slab->lock);

	if (addr == NULL)
		goto cleanup;

	void** new_head = addr;
	*new_head = slab->head;
	slab->head = new_head;

cleanup:
	spin_free(&slab->lock);
}

// Finds a suitable slab that can contain `size` bytes.
// Returns `NULL` if none can do so.
static inline Slab* slab_find_size(usize size)
{
	for (usize i = 0; i < ARRAY_SIZE(slabs); i++)
	{
		if (slabs[i].ent_size >= size)
			return &slabs[i];
	}
	return NULL;
}

void* slab_alloc(usize size)
{
	// Find a suitable slab.
	Slab* slab = slab_find_size(size);
	// If there is already a usable slab, do allocation on that.
	if (slab != NULL)
		return slab_do_alloc(slab);

	// Get how many pages have to be allocated in order to fit `size`.
	usize num_pages = ROUND_UP(size, CONFIG_page_size);
	// Allocate the pages plus an additional page for metadata.
	PhysAddr ret = pm_alloc(num_pages + 1);
	// If the allocation failed, return NULL.
	if (ret == 0)
		return NULL;

	ret = ret + (PhysAddr)pm_get_phys_base();
	// Write metadata into the first page.
	SlabInfo* info = (SlabInfo*)ret;
	info->num_pages = num_pages;
	info->size = size;
	// Skip the first page and return the next one.
	return (void*)(ret + CONFIG_page_size);
}

void* slab_realloc(void* old, usize new_bytes)
{
	// If we didn't get a previous size, just treat this like a regular allocation.
	if (old == NULL)
		return slab_alloc(new_bytes);

	// If the address is page aligned.
	if ((usize)old == ALIGN_DOWN((usize)old, CONFIG_page_size))
	{
		SlabInfo* info = (SlabInfo*)(old - CONFIG_page_size);
		if (ROUND_UP(info->size, CONFIG_page_size) == ROUND_UP(new_bytes, CONFIG_page_size))
		{
			info->size = new_bytes;
			return old;
		}
		void* new = slab_alloc(new_bytes);
		if (new == NULL)
			return NULL;

		// Copy the old data over to the new memory.
		if (info->size > new_bytes)
			memcpy(new, old, new_bytes);
		else
			memcpy(new, old, info->size);

		slab_free(old);
		return new;
	}

	// Filter out the meta data.
	SlabHeader* slab_header = (SlabHeader*)((usize)old & ~0xFFF);
	Slab* slab = slab_header->slab;

	if (new_bytes > slab->ent_size)
	{
		void* new_addr = slab_alloc(new_bytes);
		if (new_addr == NULL)
			return NULL;

		memcpy(new_addr, old, slab->ent_size);
		slab_do_free(slab, old);
		return new_addr;
	}

	return old;
}

void slab_free(void* addr)
{
	if (addr == NULL)
		return;

	// If the address is page aligned.
	if ((usize)addr == ALIGN_DOWN((usize)addr, CONFIG_page_size))
	{
		SlabInfo* info = (SlabInfo*)(addr - CONFIG_page_size);
		pm_free(((PhysAddr)info - (PhysAddr)pm_get_phys_base()), info->num_pages + 1);
		return;
	}

	SlabHeader* header = (SlabHeader*)(ALIGN_DOWN((usize)addr, CONFIG_page_size));
	slab_do_free(header->slab, addr);
}
