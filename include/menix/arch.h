// Architecture dependent platform init and deinit

#pragma once

#include <menix/boot.h>

#include <bits/arch.h>
#include <bits/asm.h>

typedef struct ArchCpu Cpu;
typedef struct ArchRegisters CpuRegisters;

// Initializes the platform for use by the kernel and boot routines.
void arch_early_init();

// Initializes the rest of the platform after the boot routines have completed.
void arch_init(BootInfo* info);

// Halts all CPUs.
void arch_stop(BootInfo* info);

// Writes all registers to the current output stream.
void arch_dump_registers();

extern Cpu* cpus;

// Gets the current CPU. Synopsis:
// Cpu* arch_current_cpu(void);
#ifndef arch_current_cpu
#error "Architecture should define arch_current_cpu"
#endif
