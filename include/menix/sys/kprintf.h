#ifndef _MENIX_SYS_KPRINTF_H
#define _MENIX_SYS_KPRINTF_H

#include <menix/util/attributes.h>

#define pr_crit(fmt, ...) kprintf(0, fmt, ##__VA_ARGS__)
#define pr_err(fmt, ...)  kprintf(1, fmt, ##__VA_ARGS__)
#define pr_warn(fmt, ...) kprintf(2, fmt, ##__VA_ARGS__)
#define pr_log(fmt, ...)  kprintf(3, fmt, ##__VA_ARGS__)
#define pr_dbg(fmt, ...)  kprintf(4, fmt, ##__VA_ARGS__)

[[__format(printf, 2, 3)]]
void kprintf(int severity, const char* message, ...);

#endif
