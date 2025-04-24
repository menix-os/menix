// Make uACPI use our own functions.

#pragma once

#include <stddef.h>

void *memcpy(void *__restrict dst, const void *__restrict src, size_t len);
void *memmove(void *dst, const void *src, size_t len);
void *memset(void *dst, int val, size_t len);
int memcmp(const void *s1, const void *s2, size_t len);

#define uacpi_memcpy memcpy
#define uacpi_memmove memmove
#define uacpi_memset memset
#define uacpi_memcmp memcmp
