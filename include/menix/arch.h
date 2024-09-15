// Architecture dependent platform init and deinit

#pragma once

#include <menix/boot.h>

#include <bits/arch.h>
#include <bits/asm.h>

#define arch_log(fmt, ...) kmesg("[" CONFIG_arch "]\t" fmt, ##__VA_ARGS__)

// CPU-local information.
typedef struct Cpu
{
	usize id;				  // Unique ID of this CPU.
	usize kernel_stack;		  // Stack pointer for the kernel.
	usize user_stack;		  // Stack pointer for the user space.
	struct Thread* thread;	  // Current thread running on this CPU.
	usize ticks_active;		  // The amount of ticks this thread has been active.

	// Architecture dependent information.
#ifdef CONFIG_arch_x86
	TaskStateSegment tss;
	u32 lapic_id;					   // Local APIC ID.
	usize fpu_size;					   // Size of the FPU in bytes.
	void (*fpu_save)(void* dst);	   // Function to call when saving the FPU state.
	void (*fpu_restore)(void* dst);	   // Function to call when restoring the FPU state.
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

// Writes the contents of all registers to regs.
void arch_get_registers(CpuRegisters* regs);

// Writes all registers to the current output stream.
void arch_dump_registers(CpuRegisters* regs);

// Gets processor metadata.
Cpu* arch_current_cpu();

// Jump to user mode and set the instruction pointer to `ip`.
extern void arch_return_to_user(VirtAddr ip);
