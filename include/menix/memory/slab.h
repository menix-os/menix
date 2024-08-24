// SLAB memory allocator

#pragma once

#include <menix/common.h>
#include <menix/thread/spin.h>

typedef struct
{
	usize num_pages;	// Amount of pages connected to this slab.
	usize size;			// Size of this slab.
} SlabInfo;

typedef struct
{
	SpinLock lock;	   // Access lock.
	usize ent_size;	   // Size of one entry.
	void** head;
} Slab;

typedef struct
{
	Slab* slab;
} SlabHeader;

// Initializes the SLAB structures.
void slab_init();
void* slab_alloc(usize size);
void* slab_realloc(void* old, usize size);
void slab_free(void* address);
