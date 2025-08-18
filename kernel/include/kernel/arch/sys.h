#ifndef _KERNEL_ARCH_SYS_H
#define _KERNEL_ARCH_SYS_H

// Initializes the boot processor.
void arch_bsp_init();

// Stop all execution upon panic.
[[noreturn]]
void arch_panic();

#endif
