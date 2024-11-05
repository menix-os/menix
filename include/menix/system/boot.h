// Entry point and boot procedures.

#pragma once

#include <menix/common.h>
#include <menix/memory/pm.h>
#include <menix/util/log.h>

#ifdef CONFIG_acpi
#include <menix/system/acpi/types.h>
#endif

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
	const char* cmd;		   // Command line.
	usize file_num;			   // Amount of files loaded.
	BootFile* files;		   // Array of files.
	usize mm_num;			   // Amount of memory map entries.
	PhysMemory* memory_map;	   // Physical memory mapping.
	void* kernel_virt;		   // Virtual address of the kernel.
	PhysAddr kernel_phys;	   // Physical address of the kernel.
	void* phys_map;			   // Memory mapped lower memory address.

#ifdef CONFIG_smp
	usize cpu_num;				// Amount of processors detected.
	atomic usize cpu_active;	// Amount of processors active.
	usize boot_cpu;				// Index of the processor that was used to boot.
	Cpu* cpus;					// CPU information.
#endif
#ifdef CONFIG_acpi
	AcpiRsdp* acpi_rsdp;	// ACPI RSDP table.
#endif
#ifdef CONFIG_open_firmware
	void* fdt_blob;	   // Device tree blob.
#endif
} BootInfo;

typedef enum
{
	ShutdownReason_Normal = 0,
	ShutdownReason_Unknown = 1,
	ShutdownReason_Abort = 2,
} ShutdownReason;

// Initializes common kernel systems.
// This function can be called as soon as the following functions have been called (in order):
// `arch_early_init`, `pm_init`, `vm_init`, `alloc_init`.
void kernel_early_init();

// Initializes the rest of the system after booting has finished.
// This function can be called as soon as the following functions have been called (in order):
// `kernel_early_init`, `fw_init`, `arch_init`.
void kernel_init();

// Gets called after platform initialization has finished.
// This is the main kernel function.
ATTR(noreturn) void kernel_main();

// Gets called if a shutdown was requested by the firmware, a syscall or if the init program terminated.
ATTR(noreturn) void kernel_shutdown(ShutdownReason reason);
