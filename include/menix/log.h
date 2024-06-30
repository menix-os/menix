//? Kernel logging

#pragma once

#include <menix/common.h>

#define LOG_DEBUG 0
#define LOG_INFO  1
#define LOG_WARN  2
#define LOG_ERR	  3

// Print a message to the kernel log.
void kmesg(int32_t level, const char* fmt, ...);
