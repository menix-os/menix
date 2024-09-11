// x86 physical memory allocator.

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/memory/pm.h>
#include <menix/thread/spin.h>
#include <menix/util/bitmap.h>

#include <string.h>

static SpinLock lock;
static BitMap bit_map = NULL;		// This bitmap stores whether a page is in use or not.
static void* phys_addr = NULL;		// Memory mapped lower 4GiB physical memory. This is only used to store the bitmap.
static usize num_pages = 0;			// Total amount of available pages.
static usize num_free_pages = 0;	// Amount of unused pages.
static usize last_page = 0;			// The last page marked as used.

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

	arch_log("Initialized physical memory management, free memory = %u MiB\n",
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

void* pm_get_phys_base()
{
	return phys_addr;
}

static PhysAddr get_free_pages(usize amount, usize start)
{
	usize i = start;
	usize range_start = 0;

	// Get a region of consecutive pages that fulfill the requested amount.
	while (i < num_pages)
	{
		// If this page is used, skip it.
		if (bitmap_get(bit_map, i))
			goto next_page;

		range_start = i;

		// Otherwise, check if the next pages are free as well.
		// Start with the page after `i`.
		for (usize j = 1; j < amount; j++)
		{
			if (bitmap_get(bit_map, range_start + j))
				goto next_page;
		}

		// If we got here, that means we have found a region with `amount` consecutive pages.
		for (usize x = 0; x < amount; x++)
		{
			bitmap_set(bit_map, range_start + x);
		}

		last_page = range_start + amount + 1;
		return (PhysAddr)(range_start * CONFIG_page_size);

next_page:
		i++;
	}

	return 0;
}

PhysAddr pm_arch_alloc(usize amount)
{
	spin_acquire_force(&lock);

	PhysAddr mem = get_free_pages(amount, last_page);
	// If we couldn't find a free region starting at our last page offset, do another check, but from the beginning.
	// This is a lot slower, but a last resort because the other option is to panic as we are out of physical memory.
	if (mem == 0)
	{
		kassert(num_free_pages > 0, "Out of physical memory!");
		last_page = 0;
		mem = get_free_pages(amount, last_page);
	}

	kassert(mem != 0, "Unable to allocate %zu consecutive pages, total %zu available!", amount, num_free_pages);

	// Lastly, mark the pages as used now.
	num_free_pages -= amount;

	spin_free(&lock);
	return mem;
}

void pm_arch_free(PhysAddr addr, usize amount)
{
	spin_acquire_force(&lock);

	// Mark the page(s) as free.
	const usize page_idx = addr / CONFIG_page_size;
	for (usize i = page_idx; i < page_idx + amount; i++)
	{
		bitmap_clear(bit_map, i);
	}
	num_free_pages += 1;

	spin_free(&lock);
}
