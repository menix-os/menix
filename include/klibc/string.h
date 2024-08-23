// Kernel C library - string.h

#pragma once

#include <menix/common.h>

i32 memcmp(const void* s1, const void* s2, usize len);
void* memcpy(void* restrict dst, const void* restrict src, usize len);
void* memmove(void* dst, const void* src, usize len);
void* memset(void* dst, u8 val, usize len);
usize strlen(const char* src);
usize strnlen(const char* src, usize len);
