#ifndef _KERNEL_ARCH_SYS_H
#define _KERNEL_ARCH_SYS_H

#include <kernel/sys/percpu.h>

// Initializes the boot processor.
void arch_bsp_early_init();

// Initializes a CPU.
void arch_cpu_init(struct percpu* cpu);

// Stop all execution upon panic.
[[noreturn]]
void arch_panic();

#endif
