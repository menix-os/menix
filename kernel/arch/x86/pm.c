// x86 physical memory allocator.

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/thread/spin.h>

#include <string.h>

// Get the bit at `bit`.
#define pm_arch_bit_get(map, bit)	((u8*)map)[bit / 8] & (1 << (bit % 8))
// Enable the bit at `bit`.
#define pm_arch_bit_set(map, bit)	((u8*)map)[bit / 8] |= (1 << (bit % 8))
// Disable the bit at `bit`.
#define pm_arch_bit_clear(map, bit) ((u8*)map)[bit / 8] &= ~(1 << (bit % 8))

static SpinLock lock = {0};
static void* phys_addr = NULL;		// Memory mapped physical memory.
static void* bit_map = NULL;		// The bitmap stores booleans that describe whether a page is in use or not.
static usize num_pages = 0;			// Total amount of available pages.
static usize num_free_pages = 0;	// Amount of unused pages.

void pm_init(void* phys_base, PhysMemory* mem_map, usize num_entries)
{
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
			pm_arch_bit_clear(bit_map, (mem_map[i].address + j) / CONFIG_page_size);
			num_free_pages++;
		}
	}
}

void* pm_arch_alloc(usize amount)
{
	spin_acquire_force(&lock);

	spin_free(&lock);
	return NULL;
}
