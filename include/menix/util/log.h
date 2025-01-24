// Kernel logging

#pragma once

#include <menix/common.h>
#include <menix/fs/handle.h>
#include <menix/system/arch.h>
#include <menix/system/sch/thread.h>
#include <menix/system/time/clock.h>

#if !defined(NDEBUG)
#define kassert(expr, msg, ...) \
	do \
	{ \
		if (!(expr)) \
		{ \
			print_error("Assertion failed!\n" \
						"[Location]\t%s (%s:%u)\n" \
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

#define __get_print_info \
	usize __time = clock_get_elapsed(); \
	usize __secs = (__time / 1000000000); \
	usize __millis = ((__time / 1000) % 1000000); \
	CpuInfo* __cpu = arch_current_cpu(); \
	usize __tid = 0; \
	if (__cpu != NULL && __cpu->thread != NULL) \
		__tid = __cpu->thread->id;

#define print_log(fmt, ...) \
	do \
	{ \
		__get_print_info; \
		kmesg_direct("[%5zu.%06zu] [%7zu] " fmt, __secs, __millis, __tid, ##__VA_ARGS__); \
	} while (0)

#define print_warn(fmt, ...) \
	do \
	{ \
		__get_print_info; \
		kmesg_direct("[%5zu.%06zu] [%7zu] warn: " fmt, __secs, __millis, __tid, ##__VA_ARGS__); \
	} while (0)

#define print_error(fmt, ...) \
	do \
	{ \
		__get_print_info; \
		kmesg_direct("[%5zu.%06zu] [%7zu] error: " fmt, __secs, __millis, __tid, ##__VA_ARGS__); \
	} while (0)

#define todo() \
	do \
	{ \
		__get_print_info; \
		kmesg_direct("[%5zu.%06zu] [%7zu] warn: %s is still TODO!\n", __secs, __millis, __tid, __FUNCTION__); \
	} while (0)

// Print a message to the kernel log.
void kmesg_direct(const char* fmt, ...);

typedef struct Context Context;

// Print a stack trace to the kernel log.
void ktrace(Context* regs);

// Abort kernel execution.
ATTR(noreturn) void kabort();
