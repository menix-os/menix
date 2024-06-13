/*------------------------------------------
Kernel C library - "string.h" implementation
------------------------------------------*/

#include <menix/string.h>

size_t strlen(const char* str)
{
	size_t result = 0;
	while (*str++)
	{
		result++;
	}
	return result;
}

int memcmp(const void* s1, const void* s2, size_t size)
{
	int	  diff = 0;
	char* s1ptr = (char*)s1;
	char* s2ptr = (char*)s2;

	for (size_t i = 0; i < size; i++)
	{
		if (s1ptr[i] != s2ptr[i])
			diff++;
	}
	return diff;
}

void* memmove(void* dstptr, const void* srcptr, size_t size)
{
	uint8_t*	   dst = (uint8_t*)dstptr;
	const uint8_t* src = (const uint8_t*)srcptr;
	if (dst < src)
	{
		for (size_t i = 0; i < size; i++)
			dst[i] = src[i];
	}
	else
	{
		for (size_t i = size; i != 0; i--)
			dst[i - 1] = src[i - 1];
	}
	return dstptr;
}

void* memset(void* bufptr, int value, size_t size)
{
	uint8_t* buf = (uint8_t*)bufptr;
	for (size_t i = 0; i < size; i++)
		buf[i] = (uint8_t)value;
	return bufptr;
}
