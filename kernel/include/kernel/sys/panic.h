#ifndef _KERNEL_SYS_PANIC_H
#define _KERNEL_SYS_PANIC_H

#include <kernel/util/attributes.h>

[[noreturn]]
void panic();

#endif
