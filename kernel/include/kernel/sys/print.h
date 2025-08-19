#ifndef _KERNEL_SYS_PRINT_H
#define _KERNEL_SYS_PRINT_H

#include <kernel/util/compiler.h>

[[__format(printf, 1, 2)]]
void kprintf(const char* message, ...);

#endif
