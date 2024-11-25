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
typedef struct
{
	// Required for `kernel_early_init`.
	PhysMemory* memory_map;	   // Physical memory mappings.
	usize mm_num;			   // Amount of memory map entries.
	void* kernel_virt;		   // Virtual address of the kernel.
	PhysAddr kernel_phys;	   // Physical address of the kernel.
	void* phys_base;		   // Virtual base address for an identity mapping of physical memory.

	// Required fo `fw_init`.
	const char* cmd;	// Command line.
#ifdef CONFIG_acpi
	void* acpi_rsdp;	// ACPI RSDP table.
#endif
#ifdef CONFIG_open_firmware
	void* fdt_blob;	   // Device tree blob.
#endif

	// Required for `kernel_init`.
	usize file_num;				// Amount of files loaded.
	BootFile* files;			// Array of files.
	usize cpu_num;				// Amount of processors detected.
	atomic usize cpu_active;	// Amount of processors active.
	usize boot_cpu;				// Index of the processor that was used to boot.
	Cpu* cpus;					// Per-CPU information.
} BootInfo;

typedef enum
{
	ShutdownReason_Normal = 0,
	ShutdownReason_Unknown = 1,
	ShutdownReason_Abort = 2,
} ShutdownReason;

// Initializes common kernel systems.
// This function can be called as soon as `arch_early_init` has been called and `info` contains a memory map.
void kernel_early_init(BootInfo* info);

// Initializes the rest of the system after booting has finished.
// After this call, `info` is inaccessible.
// This function can be called as soon as the following functions have been called (in order):
// `kernel_early_init`, `fw_init`, `arch_init`
void kernel_init(BootInfo* info);

// Gets called after platform initialization has finished.
// This is the main kernel function.
ATTR(noreturn) void kernel_main();
