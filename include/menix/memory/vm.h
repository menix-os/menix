// Virtual memory management

#pragma once
#include <menix/common.h>

// TODO
typedef struct
{
	void* phys_addr;

} PhysMemoryBlock;

// Contains information about available physical memory.
// The boot function is responsible for providing this information before passing control to the kernel.
typedef struct
{
	size_t num_blocks;
	PhysMemoryBlock blocks;
} PhysMemoryMap;
