// Architecture dependent platform init and deinit

#pragma once

#include <menix/boot.h>

#include <bits/arch.h>
#include <bits/asm.h>

// Initializes the platform for use by the kernel and boot routines.
void arch_early_init();

// Initializes the rest of the platform after the boot routines have completed.
void arch_init(BootInfo* info);

// Halts all CPUs.
void arch_stop(BootInfo* info);

// Writes all registers to the current output stream.
ATTR(always_inline) void arch_dump_registers();
