// Architecture dependent platform init and deinit

#pragma once

#include <menix/boot.h>

#include <bits/arch.h>
#include <bits/asm.h>

// Initializes the CPU for use by the kernel and boot routines.
void arch_early_init();

// Initializes the rest of the CPU after the boot routines have completed.
void arch_init(BootInfo* info);
