//? Kernel logging

#pragma once

#include <menix/common.h>

#define LOG_DEBUG 0
#define LOG_INFO  1
#define LOG_WARN  2
#define LOG_ERR	  3

#define kmesg_cat(level, cat, fmt, ...) kmesg(level, "[" cat "\t] " fmt, ##__VA_ARGS__)
#define kassert(expr, msg) \
	if (!(expr)) \
	{ \
		kmesg_cat(LOG_ERR, "ASSERT", msg "\nAssertion:\n\t" #expr "\n" __FILE__ ":" __PASTE_STR(__LINE__) "\n"); \
	}

// Print a message to the kernel log.
void kmesg(int32_t level, const char* fmt, ...);
