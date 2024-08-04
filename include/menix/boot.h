// Entry point and boot procedures.

#pragma once

#include <menix/common.h>
#include <menix/log.h>
#include <menix/memory/vm.h>
#include <menix/video/fb.h>

#include "menix/memory/pm.h"

#define boot_log(fmt, ...) kmesg("[Boot] " fmt, ##__VA_ARGS__)

typedef struct
{
	void* address;	  // Start of the file
	usize size;		  // Size of the file
	char* path;		  // Path of the file
} BootFile;

// Information provided to the kernel by the boot protocol.
typedef struct
{
	const char* cmd;		   // Command line
	usize fb_num;			   // Amount of frame buffers
	FrameBuffer* fb;		   // Available frame buffer(s)
	usize pm_num;			   // Amount of memory map entries
	PhysMemory* memory_map;	   // Physical memory mapping
	usize file_num;			   // Amount of files loaded
	BootFile* files;		   // Available files
#ifdef CONFIG_acpi
	void* acpi_rsdp;	// ACPI RSDP table.
#endif
#ifdef CONFIG_open_firmware
	void* fdt_blob;	   // Device tree blob.
#endif
} BootInfo;

// Gets called after platform initialization has finished.
// This is the main kernel function.
void kernel_main(BootInfo* const info);
