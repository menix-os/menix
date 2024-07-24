// Kernel C library - "stdio.h" implementation

#include <menix/serial.h>

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static bool print(const char* data, usize length)
{
	const u8* bytes = (const u8*)data;
	for (usize i = 0; i < length; i++)
		if (putchar(bytes[i]) == EOF)
			return false;
	return true;
}

i32 putchar(i32 ic)
{
	char c = (char)ic;
	serial_write(&c, sizeof(c));
	return ic;
}

i32 puts(const char* str)
{
	const usize written = strlen(str);
	serial_write(str, written);
	return written;
}

// TODO: The *printf family needs a proper rewrite that is more accurate.
//       Or look into completely replacing the klibc interface and use chained format calls.
//       Then we could get rid of variadic arguments which is conceptually unsafe.
i32 vprintf(const char* restrict fmt, va_list args)
{
	// Amount of bytes written.
	i32 written = 0;

	while (*fmt != '\0')
	{
		usize maxrem = INT32_MAX - written;

		if (fmt[0] != '%' || fmt[1] == '%')
		{
			if (fmt[0] == '%')
				fmt++;
			usize amount = 1;
			while (fmt[amount] && fmt[amount] != '%')
				amount++;
			if (maxrem < amount)
			{
				// TODO: Set errno to EOVERFLOW.
				return -1;
			}
			if (!print(fmt, amount))
				return -1;
			fmt += amount;
			written += amount;
			continue;
		}

		const char* format_begun_at = fmt++;
		bool write_prefix = false;
		// i32		write_limit = -1;

check_fmt:
		switch (*fmt)
		{
			// Format prefixes.
			case '#':
			{
				fmt++;
				write_prefix = true;
				goto check_fmt;
			}
			case '.':
			{
				fmt++;
				goto check_fmt;
			}
			// Character
			case 'c':
			{
				fmt++;
				char c = (char)va_arg(args, i32);

				if (!print(&c, sizeof(c)))
					return -1;

				written++;
				break;
			}
			// String of characters
			case 's':
			{
				fmt++;
				const char* str = va_arg(args, const char*);
				if (!str)
					str = "(null)";
				usize len = strlen(str);
				if (!print(str, len))
					return -1;

				written += len;
				break;
			}
			case 'i':
			case 'd':
			{
				fmt++;
				const i32 num = va_arg(args, i32);

				// The largest signed integer is 2^32, which uses
				// 10 digits + NUL.
				char str[10 + 1];
				itoa(num, str, 10);
				const usize len = strlen(str);
				if (!print(str, len))
					return -1;

				written += len;
				break;
			}
			case 'u':
			{
				fmt++;
				const u32 num = va_arg(args, u32);

				// The largest signed integer is 2^32, which uses
				// 10 digits + NUL.
				char str[10 + 1];
				utoa(num, str, 10);
				const usize len = strlen(str);
				if (!print(str, len))
					return -1;

				written += len;
				break;
			}
			case 'x':
			{
				fmt++;
				const u32 num = va_arg(args, u32);

				char str[sizeof(u32) * 2 + 1];
				itoa(num, str, 16);

				// Make letters lowercase.
				for (u32 i = 0; i < sizeof(str); i++)
				{
					if (str[i] >= 'A' && str[i] <= 'F')
						str[i] ^= 0x20;
				}

				const usize len = strlen(str);

				// Print prefix if '#' was previous format.
				if (write_prefix)
					if (!print("0x", 2))
						return -1;
				if (!print(str, len))
					return -1;

				written += len;
				break;
			}
			case 'X':
			{
				fmt++;
				const u32 num = va_arg(args, u32);

				char str[sizeof(u32) * 2 + 1];
				itoa(num, str, 16);

				const usize len = strlen(str);

				// Print prefix if '#' was previous format.
				if (write_prefix)
					if (!print("0x", 2))
						return -1;
				if (!print(str, len))
					return -1;

				written += len;
				break;
			}
			case 'p':
			{
				fmt++;
				const usize num = va_arg(args, usize);

				// Get the hex value, but with all other bytes explicitly written out.
				const usize buf_size = sizeof(usize) * 2 + 1;
				char str[buf_size];
				itoa(num, str, 16);
				const usize len = strlen(str);
				for (int i = len; i < buf_size; i++)
					str[i] = '0';
				if (!print(str, buf_size))
					return -1;

				written += len;
				break;
			}
			case 'n':
			{
				fmt++;
				i32* const ptr = va_arg(args, i32*);
				*ptr = written;
				break;
			}
			default:
			{
				fmt = format_begun_at;
				usize len = strlen(fmt);

				if (!print(fmt, len))
					return -1;

				written += len;
				fmt += len;
				break;
			}
		}
	}
	return written;
}

i32 printf(const char* restrict fmt, ...)
{
	va_list args;
	va_start(args, fmt);

	i32 written = vprintf(fmt, args);

	va_end(args);
	return written;
}
