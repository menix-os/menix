// Virtual memory management

#pragma once
#include <menix/common.h>

typedef enum : usize
{
	PhysMemoryUsage_Free,		 // Free and usable memory.
	PhysMemoryUsage_Reserved,	 // Reserved memory.
	PhysMemoryUsage_Unknown,	 // Unknown memory region.
} PhysMemoryUsage;

// Describes a single block of physical memory.
typedef struct
{
	usize address;			  // Start address of the memory region.
	usize length;			  // Length of the memory region in bytes.
	PhysMemoryUsage usage;	  // How this memory region is used.
} PhysMemory;

// Contains information about available physical memory.
// The boot function is responsible for providing this information before passing control to the kernel.
typedef struct
{
	usize num_blocks;	   // Amount of memory blocks.
	PhysMemory* blocks;	   // Array of `num_blocks` size.
} PhysMemoryMap;
