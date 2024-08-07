// x86 physical memory allocator.

#include <menix/common.h>
#include <menix/log.h>
#include <menix/memory/pm.h>
#include <menix/thread/spin.h>
#include <menix/util/bitmap.h>

#include <string.h>

#include "generated/config.h"
#include "menix/util/types.h"

static SpinLock lock;
static BitMap bit_map = NULL;		// This bitmap stores whether a page is in use or not.
static void* phys_addr = NULL;		// Memory mapped lower 4GiB physical memory. This is only used to store the bitmap.
static usize num_pages = 0;			// Total amount of available pages.
static usize num_free_pages = 0;	// Amount of unused pages.

void pm_init(void* phys_base, PhysMemory* mem_map, usize num_entries)
{
	lock = spin_new();
	phys_addr = phys_base;

	// Check for the highest usable physical memory address, so we know how much memory to allocate for the bitmap.
	usize highest = 0;
	for (usize i = 0; i < num_entries; i++)
	{
		// Only care about memory that we are able to own.
		if (mem_map[i].usage != PhysMemoryUsage_Free)
			continue;

		// Record the last byte of the current region if its address is the highest yet.
		const usize region_end = mem_map[i].address + mem_map[i].length;
		if (region_end > highest)
			highest = region_end;
	}

	num_pages = highest / CONFIG_page_size;
	const usize map_size = ALIGN_UP(num_pages / 8, CONFIG_page_size);

	// Get a memory region large enough to contain the bitmap.
	for (usize i = 0; i < num_entries; i++)
	{
		// Only care about memory that we are able to own.
		if (mem_map[i].usage != PhysMemoryUsage_Free)
			continue;

		if (mem_map[i].length >= map_size)
		{
			bit_map = phys_addr + mem_map[i].address;
			// The region where the bitmap is stored is inaccessible now.
			// * We could mark an entire page as used, but that would be wasteful.
			mem_map[i].address += map_size;
			mem_map[i].length -= map_size;
			break;
		}
	}

	// Mark all pages as used.
	memset(bit_map, 0xFF, map_size);

	for (usize i = 0; i < num_entries; i++)
	{
		// Only care about memory that we are able to own.
		if (mem_map[i].usage != PhysMemoryUsage_Free)
			continue;

		for (usize j = 0; j < mem_map[i].length; j += CONFIG_page_size)
		{
			// Mark the actual free pages as unused.
			bitmap_clear(bit_map, (mem_map[i].address + j) / CONFIG_page_size);
			num_free_pages++;
		}
	}

	kmesg("Initialized physical memory management\n    Free memory = %u MiB\n",
		  (num_free_pages * CONFIG_page_size) / MiB);
}

void pm_update_phys_base(void* phys_base)
{
	// Get the physical address of the bit_map variable.
	PhysAddr bit_map_offset = (void*)bit_map - phys_addr;
	// Add the new offset back.
	bit_map = phys_base + bit_map_offset;
	// Update the physical base.
	phys_addr = phys_base;
}

PhysAddr pm_arch_alloc(usize amount)
{
	spin_acquire_force(&lock);

	kassert(num_free_pages != 0, "Out of physical memory!\n");

	// Get a region of consecutive pages that fulfill the requested amount.
	PhysAddr mem = 0;
	usize i;
	for (i = 0; i + (amount - 1) < num_pages; i++)
	{
		// If this page is used, skip it.
		if (bitmap_get(bit_map, i))
			continue;
		else
		{
			// Otherwise, check if the next pages are free as well.
			// Start with the page after `i`.
			for (usize j = i + 1; j < amount; j++)
			{
				if (bitmap_get(bit_map, j))
					continue;
			}
			// If we got here, that means we have found a region with `amount` consecutive pages.
			mem = (PhysAddr)(i * CONFIG_page_size);
			break;
		}
	}

	kassert(mem != 0, "Unable to allocate sufficient pages!\n");

	// Lastly, mark the pages as used now.
	for (usize x = i; x < i + amount; x++)
		bitmap_set(bit_map, x);
	num_free_pages -= amount;

	spin_free(&lock);
	return mem;
}

void pm_arch_free(PhysAddr addr)
{
	// Mark the page as free.
	bitmap_clear(bit_map, addr / CONFIG_page_size);
	num_free_pages += 1;
}
