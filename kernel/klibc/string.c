// Kernel C library - "string.h" implementation

#include <menix/memory/alloc.h>

#include <string.h>

usize strlen(const char* str)
{
	usize result = 0;
	while (*str++)
	{
		result++;
	}
	return result;
}

usize strnlen(const char* str, usize len)
{
	usize result = 0;
	while (result < len && *str++)
	{
		result++;
	}
	return result;
}

int memcmp(const void* s1, const void* s2, usize size)
{
	int diff = 0;
	char* s1ptr = (char*)s1;
	char* s2ptr = (char*)s2;

	for (usize i = 0; i < size; i++)
	{
		if (s1ptr[i] != s2ptr[i])
			diff++;
	}
	return diff;
}

void* memcpy(void* restrict dst_ptr, const void* restrict src_ptr, usize size)
{
	u8* dst = (u8*)dst_ptr;
	const u8* src = (const u8*)src_ptr;
	for (usize i = 0; i < size; i++)
		dst[i] = src[i];
	return dst_ptr;
}

void* memcpy32(void* restrict dst_ptr, const void* restrict src_ptr, usize size)
{
	u32* dst = (u32*)dst_ptr;
	const u32* src = (const u32*)src_ptr;
	for (usize i = 0; i < size; i++)
		dst[i] = src[i];
	return dst_ptr;
}

void* memmove(void* dstptr, const void* srcptr, usize size)
{
	u8* dst = (u8*)dstptr;
	const u8* src = (const u8*)srcptr;
	if (dst < src)
	{
		for (usize i = 0; i < size; i++)
			dst[i] = src[i];
	}
	else
	{
		for (usize i = size; i != 0; i--)
			dst[i - 1] = src[i - 1];
	}
	return dstptr;
}

void* memset(void* bufptr, u8 value, usize size)
{
	u8* buf = (u8*)bufptr;
	for (usize i = 0; i < size; i++)
		buf[i] = (u8)value;
	return bufptr;
}

void* memset32(void* bufptr, u32 value, usize size)
{
	u32* buf = (u32*)bufptr;
	for (usize i = 0; i < size; i++)
		buf[i] = value;
	return bufptr;
}

char* strdup(const char* src)
{
	if (src == NULL)
		return NULL;

	usize length = strlen(src) + 1;
	char* dest = kmalloc(length);
	if (dest == NULL)
		return NULL;
	return memcpy(dest, src, length);
}
