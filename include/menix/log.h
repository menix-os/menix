//? Kernel logging

#pragma once

#include <menix/common.h>

#ifndef NDEBUG
#define kassert(expr, msg) \
	if (!(expr)) \
	{ \
		kmesg("Assertion failed: " msg "\nExpression:\n\t" #expr "\n" __FILE__ ":" __PASTE_STR(__LINE__) "\n"); \
	}
#else
#define kassert(expr, msg) \
	if (0) \
	{ \
	}
#endif

// Print a message to the kernel log.
void kmesg(const char* fmt, ...);

// Print a kernel stack trace.
void ktrace();
