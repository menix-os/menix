// Architecture dependent platform init and deinit

#pragma once

#include <menix/boot.h>

#include <bits/arch.h>
#include <bits/asm.h>

// CPU-local information.
typedef struct Cpu
{
	usize id;				  // Unique ID of this CPU.
	struct Thread* thread;	  // Current thread running on this CPU.
	usize kernel_stack;		  // RSP for the kernel.
	usize user_stack;		  // RSP for the user space.

	// Architecture dependent information.
#ifdef CONFIG_arch_x86
	TaskStateSegment tss;
	u32 lapic_id;
	usize fpu_size;
	void (*fpu_save)(void* dst);
	void (*fpu_restore)(void* dst);
#endif
} Cpu;

// Code-visible CPU registers.
typedef struct CpuRegisters CpuRegisters;

// Initializes the platform for use by the kernel and boot routines.
void arch_early_init(BootInfo* info);

// Initializes the rest of the platform after the boot routines have completed.
void arch_init(BootInfo* info);

// Initializes a single processor.
// `info`: Information about the CPU that has to be enabled.
// `boot`: Information about the boot CPU.
void arch_init_cpu(Cpu* info, Cpu* boot);

// Safely powers off the machine.
void arch_shutdown(BootInfo* info);

// Halts all CPUs.
void arch_stop(BootInfo* info);

// Writes all registers to the current output stream.
void arch_dump_registers();

// Gets processor metadata.
Cpu* arch_current_cpu();
