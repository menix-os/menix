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
			print_error("Environment is unsound! Assertion \"%s\" failed!\n", #expr); \
			print_error("In function \"%s\" (%s:%u):\n", __FUNCTION__, __FILE__, __LINE__); \
			print_error(msg "\n", ##__VA_ARGS__); \
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
	do \
	{ \
		kmesg_direct(fmt, ##__VA_ARGS__); \
	} while (0)

#define print_warn(fmt, ...) \
	do \
	{ \
		kmesg_direct("[warn] " fmt, ##__VA_ARGS__); \
	} while (0)

#define print_error(fmt, ...) \
	do \
	{ \
		kmesg_direct("[error] " fmt, ##__VA_ARGS__); \
	} while (0)

#define todo() \
	do \
	{ \
		kmesg_direct("[warn] %s is still TODO!\n", __FUNCTION__); \
	} while (0)

// Print a message to the kernel log.
void kmesg_direct(const char* fmt, ...);

typedef struct Context Context;

// Print a stack trace to the kernel log.
void ktrace(Context* regs);

// Abort kernel execution.
ATTR(noreturn) void kabort();
