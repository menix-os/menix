// Entry point and boot procedures.

#pragma once

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/system/video/fb.h>

typedef struct
{
	void* address;	  // Start of the file
	usize size;		  // Size of the file
	char* path;		  // Path of the file
} BootFile;

// Information provided to the kernel by the boot protocol.
// Required for `kernel_init`.
typedef struct
{
	PhysMemory* memory_map;	   // Physical memory mappings.
	usize mm_num;			   // Amount of memory map entries.
	void* kernel_virt;		   // Virtual address of the kernel.
	PhysAddr kernel_phys;	   // Physical address of the kernel.
	void* phys_base;		   // Virtual base address for an identity mapping of physical memory.
	void* kernel_file;		   // Pointer to the ELF of the kernel.
	const char* cmd;		   // Command line.
	usize file_num;			   // Amount of files loaded.
	BootFile files[32];		   // Array of files.
	FrameBuffer* fb;		   // Early frame buffer.
	PhysAddr acpi_rsdp;		   // ACPI RSDP table.
	void* fdt_blob;			   // Device tree blob.
} BootInfo;

// Initializes the rest of the system after booting has finished.
// This function can be called as soon as `kernel_early_init` has been called and `info` was filled completely.
// After this call, `info` may be destroyed.
ATTR(noreturn) void kernel_init(BootInfo* info);

// This is the main kernel function.
// Gets called after platform initialization has finished.
ATTR(noreturn) void kernel_main();

// Kernel shutdown function.
ATTR(noreturn) void kernel_fini();
