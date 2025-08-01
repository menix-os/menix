#ifndef _MENIX_SYS_KPRINTF_H
#define _MENIX_SYS_KPRINTF_H

#include <menix/util/attributes.h>

#define KPRINTF_CRITICAL "\xF0"
#define KPRINTF_ERROR    "\xF1"
#define KPRINTF_WARNING  "\xF2"
#define KPRINTF_DEFAULT  "\xF3"
#define KPRINTF_DEBUG    "\xF4"

#define pr_crit(fmt, ...) kprintf(KPRINTF_CRITICAL fmt, ##__VA_ARGS__)
#define pr_err(fmt, ...)  kprintf(KPRINTF_ERROR fmt, ##__VA_ARGS__)
#define pr_warn(fmt, ...) kprintf(KPRINTF_WARNING fmt, ##__VA_ARGS__)
#define pr_log(fmt, ...)  kprintf(KPRINTF_DEFAULT fmt, ##__VA_ARGS__)
#define pr_dbg(fmt, ...)  kprintf(KPRINTF_DEBUG fmt, ##__VA_ARGS__)

[[__format(printf, 1, 2)]]
void kprintf(const char* message, ...);

#endif
