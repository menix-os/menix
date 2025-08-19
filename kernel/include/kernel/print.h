#ifndef _KERNEL_PRINT_H
#define _KERNEL_PRINT_H

#include <kernel/compiler.h>

[[__format(printf, 1, 2)]]
void kprintf(const char* message, ...);

#endif
