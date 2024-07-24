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
	int i = 0;
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
