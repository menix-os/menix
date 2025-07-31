#ifndef _MENIX_SYS_KPRINTF_H
#define _MENIX_SYS_KPRINTF_H

#include <menix/util/attributes.h>

#define klog(fmt, ...)   kprintf(fmt, ##__VA_ARGS__)
#define kwarn(fmt, ...)  kprintf("warning: " fmt, ##__VA_ARGS__);
#define kerror(fmt, ...) kprintf("error: " fmt, ##__VA_ARGS__)

[[__format(printf, 1, 2)]]
void kprintf(const char* message, ...);

#endif
