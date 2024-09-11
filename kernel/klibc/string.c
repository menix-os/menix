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

void* memset(void* dst, u8 value, usize size)
{
	u8* buf = (u8*)dst;
	for (usize i = 0; i < size; i++)
		buf[i] = (u8)value;
	return dst;
}

void* memset32(void* dst, u32 value, usize size)
{
	u32* buf = (u32*)dst;
	for (usize i = 0; i < size; i++)
		buf[i] = value;
	return dst;
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

char* strncpy(char* restrict dst, const char* restrict src, usize len)
{
	usize src_len = strnlen(src, len);

	return memcpy(dst, src, MIN(len, src_len));
}

usize strncmp(const char* str1, const char* str2, usize len)
{
	while (len && *str1 && (*str1 == *str2))
	{
		++str1;
		++str2;
		--len;
	}
	if (len == 0)
	{
		return 0;
	}
	else
	{
		return (*(unsigned char*)str1 - *(unsigned char*)str2);
	}
}
