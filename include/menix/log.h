#ifndef _MENIX_LOG_H
#define _MENIX_LOG_H

#include <menix/util.h>

#define kassert(expr, msg, ...) \
	do { \
		if (unlikely(!(expr))) { \
			kmsg("Environment is unsound! Assertion \"%s\" failed!\n", #expr); \
			kmsg("In function \"%s\" (%s:%u):\n", __FUNCTION__, __FILE__, __LINE__); \
			kmsg(msg "\n", ##__VA_ARGS__); \
			panic(); \
		} \
	} while (0)

#define klog(fmt, ...)	 kmsg(fmt, ##__VA_ARGS__)
#define kwarn(fmt, ...)	 kmsg("warning: " fmt, ##__VA_ARGS__);
#define kerror(fmt, ...) kmsg("error: " fmt, ##__VA_ARGS__)

[[gnu::format(printf, 1, 2), gnu::weak]]
void kmsg(const char* message, ...);

[[noreturn, gnu::weak]]
void panic();

#endif
