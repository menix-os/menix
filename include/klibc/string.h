// Kernel C library - string.h

#pragma once

#include <menix/common.h>

i32 memcmp(const void*, const void*, usize);
void* memcpy(void* restrict, const void* restrict, usize);
void* memmove(void*, const void*, usize);
void* memset(void*, u8, usize);
usize strlen(const char*);
