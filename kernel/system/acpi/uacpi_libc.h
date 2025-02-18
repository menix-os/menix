// Make uACPI use our klibc functions.
#pragma once

#include <stdint.h>
#include <stddef.h>

void* memcpy(void* restrict dst, const void* restrict src, size_t len);
void* memmove(void* dst, const void* src, size_t len);
void* memset(void* dst, int32_t val, size_t len);
int32_t memcmp(const void* s1, const void* s2, size_t len);

#define uacpi_memcpy memcpy
#define uacpi_memmove memmove
#define uacpi_memset memset
#define uacpi_memcmp memcmp
