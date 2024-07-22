//? Entry point and boot procedures.

#pragma once

#include <menix/common.h>
#include <menix/gpu/fb.h>
#include <menix/log.h>

#ifdef CONFIG_efi
#include <efi.h>
#include <efiapi.h>
#endif

#define boot_log(fmt, ...) kmesg("[Boot] " fmt, ##__VA_ARGS__)
#define boot_err(fmt, ...) kmesg("[Boot] " fmt, ##__VA_ARGS__)

typedef struct
{
	const char*	 cmd;		// Command line arguments
	size_t		 fb_num;	// Amount of frame buffers
	FrameBuffer* fb;		// Available frame buffers
#ifdef CONFIG_efi
	EFI_SYSTEM_TABLE* efi_st;	 // EFI System Table
#endif
} BootInfo;

// Main entry point. Kernel code execution starts here.
void kernel_main(BootInfo* const info);
