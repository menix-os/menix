// Physical memory management

#pragma once

#include <menix/common.h>

typedef enum : u8
{
	PhysMemoryUsage_Free,		   // Free and usable memory.
	PhysMemoryUsage_Reserved,	   // Memory reserved by the System.
	PhysMemoryUsage_Bootloader,	   // Used by boot loader structures.
	PhysMemoryUsage_Kernel,		   // Kernel and modules are loaded here.
	PhysMemoryUsage_Unknown,	   // Unknown memory region.
} PhysMemoryUsage;

// Describes a single block of physical memory.
typedef struct
{
	usize address;			  // Start address of the memory region.
	usize length;			  // Length of the memory region in bytes.
	PhysMemoryUsage usage;	  // How this memory region is used.
} PhysMemory;

// Initializes the physical memory manager.
void pm_init(void* phys_base, PhysMemory* mem_map, usize num_entries);

// Updates the base address that maps directly to lower memory.
void pm_update_phys_base(void* phys_base);

// Gets the base address that maps directly to lower memory.
void* pm_get_phys_base();

// Allocates a given `amount` of `CONFIG_page_size` sized pages.
PhysAddr pm_alloc(usize amount);

// Frees pages pointed to by `pm_arch_alloc`.
void pm_free(PhysAddr addr, usize amount);

#define MENIX_BITS_INCLUDE
#include <bits/pm.h>
#undef MENIX_BITS_INCLUDE
