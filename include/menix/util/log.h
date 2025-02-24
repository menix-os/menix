// Kernel logging

#pragma once

#include <menix/common.h>

#define kassert(expr, msg, ...) \
	do \
	{ \
		if (unlikely(!(expr))) \
		{ \
			print_error("Environment is unsound! Assertion \"%s\" failed!\n", #expr); \
			print_error("In function \"%s\" (%s:%u):\n", __FUNCTION__, __FILE__, __LINE__); \
			print_error(msg "\n", ##__VA_ARGS__); \
			ktrace(NULL); \
			panic(); \
		} \
	} while (0)

#if !defined(NDEBUG)
#define kassert_debug(expr, msg, ...) \
	do \
	{ \
		if (unlikely(!(expr))) \
		{ \
			print_error("Environment is unsound! Debug assertion \"%s\" failed!\n", #expr); \
			print_error("In function \"%s\" (%s:%u):\n", __FUNCTION__, __FILE__, __LINE__); \
			print_error(msg "\n", ##__VA_ARGS__); \
			ktrace(NULL); \
			panic(); \
		} \
	} while (0)
#else
#define kassert_debug(expr, msg, ...) \
	do \
	{ \
		(void)(expr); \
	} while (0)
#endif

#define print_log(fmt, ...) \
	do \
	{ \
		kmesg(fmt, ##__VA_ARGS__); \
	} while (0)

#define print_warn(fmt, ...) \
	do \
	{ \
		kmesg("[warn] " fmt, ##__VA_ARGS__); \
	} while (0)

#define print_error(fmt, ...) \
	do \
	{ \
		kmesg("[error] " fmt, ##__VA_ARGS__); \
	} while (0)

#define todo() \
	do \
	{ \
		kmesg("[warn] %s is still TODO!\n", __FUNCTION__); \
	} while (0)

// Print a message to the kernel log.
void kmesg(const char* fmt, ...);

typedef struct Context Context;

// Print a stack trace to the kernel log.
void ktrace(Context* regs);

// Abort kernel execution.
[[noreturn]] void panic();
