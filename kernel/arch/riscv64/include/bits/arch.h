// riscv64 specific declarations.

#pragma once

#include <menix/common.h>

#define STVEC_MODE_DIRECT 0
#define STVEC_MODE_VECTOR 1

extern void arch_int_internal();
