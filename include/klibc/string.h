// Kernel C library - string.h

#pragma once

#include <menix/common.h>

// Copies a string to a buffer. Both sizes must be known at compile time.
#define fixed_strncpy(dst, src) memcpy(dst, src, MIN(sizeof(dst), sizeof(src)))

i32 memcmp(const void* s1, const void* s2, usize len);

// Copies `len` bytes from `src` to `dst`. Pointers may not overlap.
void* memcpy(void* restrict dst, const void* restrict src, usize len);

// Copies `len` bytes from `src` to `dst`.
void* memmove(void* dst, const void* src, usize len);

// Sets `len` bytes to `val`, starting at dst.
void* memset(void* dst, u8 val, usize len);

usize strlen(const char* src);
usize strnlen(const char* src, usize len);
char* strdup(const char* src);

// Copies a string with a maximum length of `len` from `src` to `dst`. Pointers may not overlap.
char* strncpy(char* restrict dst, const char* restrict src, usize len);
usize strncmp(const char* str1, const char* str2, usize len);
