// Architecture specific operations
// Everything in this header is architecture dependent

#pragma once

// This file is included from arch/.../include/
#include <bits/arch.h>

// Initializes the CPU for use by the kernel.
void arch_init();

// Shuts off the machine.
void arch_shutdown();
