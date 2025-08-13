#ifndef _KERNEL_BOOT_MAIN_H
#define _KERNEL_BOOT_MAIN_H

// Initializes the early parts of the kernel.
void kernel_early_init();

// The kernel's main init function.
[[noreturn]]
void kernel_main();

#endif
