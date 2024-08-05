// Physical memory management

#pragma once

#include <menix/common.h>

// Represents a physical address.
typedef usize PhysAddr;

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

void pm_init(void* phys_base, PhysMemory* mem_map, usize num_entries);

#include <bits/pm.h>
