// Entry point and boot procedures.

#pragma once

#include <menix/common.h>
#include <menix/gpu/fb.h>
#include <menix/log.h>
#include <menix/memory/vm.h>

#define boot_log(fmt, ...) kmesg("[Boot] " fmt, ##__VA_ARGS__)
#define boot_err(fmt, ...) kmesg("[Boot] " fmt, ##__VA_ARGS__)

typedef struct
{
	const char* cmd;			 // Command line
	size_t fb_num;				 // Amount of frame buffers
	FrameBuffer* fb;			 // Available frame buffer(s)
	PhysMemoryMap memory_map;	 //
} BootInfo;

// Main entry point. Kernel code execution starts here.
void kernel_main(BootInfo* const info);
