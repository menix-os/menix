// Entry point and boot procedures.

#pragma once

#include <menix/common.h>
#include <menix/log.h>
#include <menix/memory/vm.h>
#include <menix/video/fb.h>

#ifdef CONFIG_efi
#include <efi.h>
#include <efiapi.h>
#endif

#define boot_log(fmt, ...) kmesg("[Boot] " fmt, ##__VA_ARGS__)
#define boot_err(fmt, ...) kmesg("[Boot] " fmt, ##__VA_ARGS__)

// Information provided to the kernel by the boot protocol.
typedef struct
{
	const char* cmd;	// Command line
	usize fb_num;		// Amount of frame buffers
	FrameBuffer* fb;	// Available frame buffer(s)
#ifdef CONFIG_efi
	EFI_SYSTEM_TABLE* efi_st;	 // EFI system table.
#endif
	PhysMemoryMap memory_map;	 // Physical memory mapping.
} BootInfo;

// Main entry point. Kernel code execution starts here.
void kernel_main(BootInfo* const info);
