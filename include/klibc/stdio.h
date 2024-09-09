// Kernel C library - stdio.h

#pragma once

#include <menix/common.h>

#include <stdarg.h>

#define EOF (-1)

i32 printf(const char* restrict fmt, ...);
i32 vprintf(const char* restrict fmt, va_list args);
