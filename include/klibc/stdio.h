//? Kernel C library - stdio.h

#pragma once

#include <stdint.h>
#include <stdarg.h>

#define EOF (-1)

int32_t printf(const char* restrict fmt, ...);
int32_t vprintf(const char* restrict fmt, va_list args);
int32_t putchar(int32_t ch);
int32_t puts(const char* str);
