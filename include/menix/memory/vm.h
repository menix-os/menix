// Virtual memory management

#pragma once

#include <menix/common.h>
#include <menix/memory/pm.h>

// Initializes the virtual memory mapping with a bootloader-provided physical memory map.
// `phys_base` must be a virtual address memory mapped to 0x0.
// `kernel_base` must be a physical address pointing to the memory where the kernel has been loaded.
void vm_init(void* phys_base, PhysAddr kernel_base, PhysMemory* mem_map, usize num_entries);

#include <bits/vm.h>
