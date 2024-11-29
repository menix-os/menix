// Kernel logging

#pragma once

#include <menix/common.h>
#include <menix/fs/handle.h>
#include <menix/system/time/clock.h>

#if !defined(NDEBUG) || CONFIG_force_asserts
#define kassert(expr, msg, ...) \
	do \
	{ \
		if (!(expr)) \
		{ \
			print_log("Assertion failed!\n" \
					  "[Location]\t%s (%s:%zu)\n" \
					  "[Expression]\t%s\n" \
					  "[Message]\t" msg "\n", \
					  __FUNCTION__, __FILE__, __LINE__, #expr, ##__VA_ARGS__); \
			ktrace(NULL); \
			kabort(); \
		} \
	} while (0)
#else
#define kassert(expr, msg, ...) \
	do \
	{ \
		(void)(expr); \
	} while (0)
#endif

#define print_log(fmt, ...) \
	kmesg_direct("[%5zu.%06zu] " fmt, (clock_get_elapsed() / 1000000000), ((clock_get_elapsed() / 1000) % 1000000), \
				 ##__VA_ARGS__)

#define print_warn(fmt, ...) \
	kmesg_direct("[%5zu.%06zu] warn: " fmt, (clock_get_elapsed() / 1000000000), \
				 ((clock_get_elapsed() / 1000) % 1000000), ##__VA_ARGS__)

#define print_error(fmt, ...) \
	kmesg_direct("[%5zu.%06zu] error: " fmt, (clock_get_elapsed() / 1000000000), \
				 ((clock_get_elapsed() / 1000) % 1000000), ##__VA_ARGS__)

// Print a message to the kernel log.
void kmesg_direct(const char* fmt, ...);

typedef struct Context Context;

// Print a stack trace to the kernel log.
void ktrace(Context* regs);

// Abort kernel execution.
ATTR(noreturn) void kabort();
