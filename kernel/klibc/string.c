// Kernel C library - "string.h" implementation

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
