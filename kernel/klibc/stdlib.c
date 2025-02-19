// Kernel C library - "stdlib.h" implementation

#include <menix/common.h>

#include <string.h>

static void reverse(char* s)
{
	int i, j;
	char c;

	for (i = 0, j = strlen(s) - 1; i < j; i++, j--)
	{
		c = s[i];
		s[i] = s[j];
		s[j] = c;
	}
}

static int pow(int x, unsigned int y)
{
	if (y == 0)
		return 1;
	else if ((y % 2) == 0)
		return pow(x, y / 2) * pow(x, y / 2);
	else
		return x * pow(x, y / 2) * pow(x, y / 2);
}

#define implement_itoa(name, type) \
	char* name(type value, char* str, u32 base) \
	{ \
		usize i; \
		type sign; \
		sign = value; \
		if (base == 10 && sign < 0) \
			value = -value; \
		i = 0; \
		do \
		{ \
			char c = value % base; \
			if (c > 9) \
				c += 7; \
			str[i++] = c + '0'; \
		} while ((value /= base) > 0); \
		if (base == 10 && sign < 0) \
			str[i++] = '-'; \
		str[i] = '\0'; \
		reverse(str); \
		return str; \
	}

#define implement_utoa(name, type) \
	char* name(type value, char* str, u32 base) \
	{ \
		usize i = 0; \
		do \
		{ \
			char c = value % base; \
			if (c > 9) \
				c += 7; \
			str[i++] = c + '0'; \
		} while ((value /= base) > 0); \
		str[i] = '\0'; \
		reverse(str); \
		return str; \
	}

#define implement_atoi(name, type) \
	type name(const char* str, u32 base) \
	{ \
		usize len = strlen(str); \
		type result = 0; \
		usize i = 0; \
		if (str[0] == '-') \
			i++; \
		for (; i < len; i++) \
			result += (str[i] - '0') * pow(base, len - i - 1); \
		if (str[0] == '-') \
			result *= -1; \
		return result; \
	}

// TODO: Hex is broken
#define implement_atou(name, type) \
	type name(const char* str, u32 base) \
	{ \
		usize len = strlen(str); \
		type result = 0; \
		usize i = 0; \
		for (; i < len; i++) \
			result += (str[i] - '0') * pow(base, len - i - 1); \
		return result; \
	}

implement_atoi(atoi8, i8);
implement_atoi(atoi16, i16);
implement_atoi(atoi32, i32);
implement_atoi(atoi64, i64);

implement_atou(atou8, u8);
implement_atou(atou16, u16);
implement_atou(atou32, u32);
implement_atou(atou64, u64);

implement_itoa(i8toa, i8);
implement_itoa(i16toa, i16);
implement_itoa(i32toa, i32);
implement_itoa(i64toa, i64);

implement_utoa(u8toa, u8);
implement_utoa(u16toa, u16);
implement_utoa(u32toa, u32);
implement_utoa(u64toa, u64);
