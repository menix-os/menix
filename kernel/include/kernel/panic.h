#ifndef _KERNEL_PANIC_H
#define _KERNEL_PANIC_H

#include <kernel/compiler.h>

// Stop all execution upon panic.
[[noreturn]]
void panic();

#endif
