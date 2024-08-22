// Kernel C library - "stdlib.h" implementation

#include <menix/common.h>

#include <stdlib.h>
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

i32 atoi(char* str, u32 base)
{
	usize len = strlen(str);
	i32 result = 0;
	usize i = 0;
	// Sign.
	if (str[0] == '-')
		i++;
	for (; i < len; i++)
	{
		result += (str[i] - '0') * pow(base, len - i - 1);
	}
	if (str[0] == '-')
		result *= -1;

	return result;
}

u32 atou(char* str, u32 base)
{
	usize len = strlen(str);
	u32 result = 0;
	usize i = 0;
	for (; i < len; i++)
	{
		result += (str[i] - '0') * pow(base, len - i - 1);
	}

	return result;
}

char* itoa(i32 value, char* str, u32 base)
{
	i32 i, sign;
	sign = value;

	if (base == 10 && sign < 0)
		value = -value;
	i = 0;
	do
	{
		char c = value % base;
		if (c > 9)
			c += 7;	   // Skip to the letters for hex. ('9' + 7 = 'A')
		str[i++] = c + '0';
	} while ((value /= base) > 0);
	if (base == 10 && sign < 0)
		str[i++] = '-';
	str[i] = '\0';
	reverse(str);
	return str;
}

char* utoa(u32 value, char* str, u32 base)
{
	i32 i = 0;
	do
	{
		char c = value % base;
		if (c > 9)
			c += 7;	   // Skip to the letters for hex. ('9' + 7 = 'A')
		str[i++] = c + '0';
	} while ((value /= base) > 0);
	str[i] = '\0';
	reverse(str);
	return str;
}

char* lutoa(u64 value, char* str, u32 base)
{
	i32 i = 0;
	do
	{
		char c = value % base;
		if (c > 9)
			c += 7;	   // Skip to the letters for hex. ('9' + 7 = 'A')
		str[i++] = c + '0';
	} while ((value /= base) > 0);
	str[i] = '\0';
	reverse(str);
	return str;
}
