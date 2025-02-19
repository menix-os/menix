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

void* memcpy(void* restrict dest, const void* restrict src, usize n)
{
	if (n == 0)
		return dest;

	usize d = (usize)dest;
	usize s = (usize)src;

	if (d % sizeof(usize) != 0 || s % sizeof(usize) != 0)
	{
		while (n && (d % sizeof(usize) != 0) && (s % sizeof(usize) != 0))
		{
			*((u8*)d) = *((u8*)s);
			d++;
			s++;
			n--;
		}
	}

	usize* qword_dest = (usize*)d;
	const usize* qword_src = (const usize*)s;
	const usize word_count = n / sizeof(usize);

	for (usize i = 0; i < word_count; i++)
	{
		qword_dest[i] = qword_src[i];
	}

	usize remaining_bytes = n % sizeof(usize);
	d = (usize)(qword_dest + word_count);
	s = (usize)(qword_src + word_count);

	while (remaining_bytes--)
	{
		*((u8*)d) = *((u8*)s);
		d++;
		s++;
	}

	return dest;
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

void* memset(void* dest, int value, usize n)
{
	if (n == 0)
		return dest;

	usize d = (usize)dest;

	if (d % sizeof(usize) != 0)
	{
		while (n && (d % sizeof(usize) != 0))
		{
			*((u8*)d) = value;
			d++;
			n--;
		}
	}

	usize* qword_dest = (usize*)d;
	usize qword_value = value;
	qword_value |= (qword_value << 8);
	qword_value |= (qword_value << 16);
	qword_value |= (qword_value << 32);

	const usize word_count = n / sizeof(usize);

	for (usize i = 0; i < word_count; i++)
	{
		qword_dest[i] = qword_value;
	}

	usize remaining_bytes = n % sizeof(usize);
	d = (usize)(qword_dest + word_count);

	while (remaining_bytes--)
	{
		*((u8*)d) = value;
		d++;
	}

	return dest;
}

char* strdup(const char* src)
{
	usize length = strlen(src) + 1;
	char* dest = kmalloc(length);
	if (dest == NULL)
		return NULL;
	return memcpy(dest, src, length);
}

char* strncpy(char* restrict dst, const char* restrict src, usize len)
{
	usize src_len = strnlen(src, len) + 1;
	return memcpy(dst, src, MIN(len, src_len));
}

int strncmp(const char* str1, const char* str2, usize len)
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
