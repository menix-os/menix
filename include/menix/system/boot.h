// Entry point and boot procedures.

#pragma once

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/util/log.h>

#define boot_log(fmt, ...) kmesg("[Boot]\t" fmt, ##__VA_ARGS__)

typedef struct
{
	void* address;	  // Start of the file
	usize size;		  // Size of the file
	char* path;		  // Path of the file
} BootFile;

typedef struct Cpu Cpu;

// Information provided to the kernel by the boot protocol.
// Required for `kernel_early_init`.
typedef struct
{
	PhysMemory* memory_map;	   // Physical memory mappings.
	usize mm_num;			   // Amount of memory map entries.
	void* kernel_virt;		   // Virtual address of the kernel.
	PhysAddr kernel_phys;	   // Physical address of the kernel.
	void* phys_base;		   // Virtual base address for an identity mapping of physical memory.
	void* kernel_file;		   // Pointer to the ELF of the kernel.
	const char* cmd;		   // Command line.
#ifdef CONFIG_acpi
	void* acpi_rsdp;	// ACPI RSDP table.
#endif
#ifdef CONFIG_open_firmware
	void* fdt_blob;	   // Device tree blob.
#endif
} EarlyBootInfo;

// Information provided to the kernel by the boot protocol.
// Required for `kernel_init`.
typedef struct
{
	usize file_num;		// Amount of files loaded.
	BootFile* files;	// Array of files.

	// TODO: Handle SMP startup ourselves.
	usize cpu_num;						 // Amount of processors detected.
	volatile atomic usize cpu_active;	 // Amount of processors active.
	usize boot_cpu;						 // Index of the processor that was used to boot.
	Cpu* cpus;							 // Per-CPU information.
} BootInfo;

// Initializes basic kernel functions.
// This function can be called as soon as `info.early_init` contains data.
void kernel_early_init(EarlyBootInfo* info);

// Initializes the rest of the system after booting has finished.
// This function can be called as soon as `kernel_early_init` has been called and `info` was filled completely.
// After this call, `info` may be destroyed.
ATTR(noreturn) void kernel_init(BootInfo* info);

// Gets called after platform initialization has finished.
// This is the main kernel function.
ATTR(noreturn) void kernel_main();
