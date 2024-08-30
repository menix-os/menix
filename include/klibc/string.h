// Kernel C library - string.h

#pragma once

#include <menix/common.h>

// Copies a string to a buffer. Both sizes must be known at compile time.
#define fixed_strncpy(dst, src) memcpy(dst, src, MIN(sizeof(dst), sizeof(src)));

i32 memcmp(const void* s1, const void* s2, usize len);

// Copies `len` bytes from `src` to dst`.
void* memcpy(void* restrict dst, const void* restrict src, usize len);

// `Copies `len` 32-bit integers from `src` to `dst`.
void* memcpy32(void* restrict dst, const void* restrict src, usize len);
void* memmove(void* dst, const void* src, usize len);
void* memset(void* dst, u8 val, usize len);
void* memset32(void* bufptr, u32 value, usize size);
usize strlen(const char* src);
usize strnlen(const char* src, usize len);
char* strdup(const char* src);
