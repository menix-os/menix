#ifndef _MENIX_SYS_PANIC_H
#define _MENIX_SYS_PANIC_H

#include <menix/util/attributes.h>

[[noreturn]]
void panic(const char* msg, ...);

#endif
