// Entry point and boot procedures.

#pragma once

#include <menix/common.h>
#include <menix/log.h>
#include <menix/memory/pm.h>
#include <menix/video/fb.h>

typedef struct
{
	void* address;	  // Start of the file
	usize size;		  // Size of the file
	char* path;		  // Path of the file
} BootFile;

// Information provided to the kernel by the boot protocol.
typedef struct
{
	const char* cmd;	// Command line

	usize fb_num;		// Amount of frame buffers
	FrameBuffer* fb;	// Available frame buffer(s)

// This is architecture dependent, but almost every architecture should have it.
#if defined(CONFIG_arch_x86) || defined(CONFIG_arch_aarch64) || defined(CONFIG_arch_riscv64)
	usize mm_num;			   // Amount of memory map entries
	PhysMemory* memory_map;	   // Physical memory mapping
	void* kernel_virt;		   // Virtual address of the kernel.
	PhysAddr kernel_phys;	   // Physical address of the kernel.
	void* phys_map;			   // Memory mapped lower memory address.
#endif

	usize file_num;		// Amount of files loaded
	BootFile* files;	// Available files

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
