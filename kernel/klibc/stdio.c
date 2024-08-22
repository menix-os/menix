// Kernel C library - "stdio.h" implementation

#include <menix/io/terminal.h>

#include <limits.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static bool print(const char* data, usize length)
{
	const char* bytes = (char*)data;
	terminal_puts(bytes, length);
	return true;
}

i32 putchar(i32 ic)
{
	const char c = ic;
	terminal_puts(&c, 1);
	return ic;
}

// TODO: The *printf family needs a proper rewrite that is more accurate.
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
				return -1;
			}
			if (!print(fmt, amount))
				return -1;
			fmt += amount;
			written += amount;
			continue;
		}

		const char* format_begun_at = fmt++;
		bool left_justify = false;
		bool preceed_sign = false;
		bool blank_sign = false;
		bool write_prefix = false;
		bool has_width = false;
		usize width = 0;
		bool has_precision = false;
		usize precision = 0;

check_fmt:
		switch (*fmt)
		{
			// Flags
			case '-':
			{
				left_justify = true;
				fmt++;
				goto check_fmt;
			}
			case '+':
			{
				preceed_sign = true;
				fmt++;
				goto check_fmt;
			}
			case ' ':
			{
				blank_sign = true;
				fmt++;
				goto check_fmt;
			}
			case '#':
			{
				write_prefix = true;
				fmt++;
				goto check_fmt;
			}
			case '0':
			{
				write_prefix = true;
				fmt++;
				goto check_fmt;
			}
			// Width
			case '1':
			case '2':
			case '3':
			case '4':
			case '5':
			case '6':
			case '7':
			case '8':
			case '9':
			{
				has_width = true;
				char number[10 + 1] = {0};
				usize idx = 0;
				while (*fmt >= '0' && *fmt <= '9')
				{
					number[idx++] = *fmt;
					fmt++;
				}
				width = atou(number, 10);

				goto check_fmt;
			}
			// Precision
			case '.':
			{
				has_precision = true;
				fmt++;
				if (*fmt == '*')
				{
					precision = (char)va_arg(args, i32);
				}
				else
				{
					char number[10 + 1] = {0};
					usize idx = 0;
					while (*fmt >= '0' && *fmt <= '9')
					{
						number[idx++] = *fmt;
						fmt++;
					}
					precision = atou(number, 10);
				}
				goto check_fmt;
			}
			// Character
			case 'c':
			{
				char c = (char)va_arg(args, i32);

				if (!print(&c, sizeof(c)))
					return -1;

				written++;
				fmt++;
				break;
			}
			// String of characters
			case 's':
			{
				const char* str = va_arg(args, const char*);
				if (!str)
					str = "(null)";
				usize len = 0;
				if (has_width)
					len = width;
				else
					len = strlen(str);

				if (!print(str, len))
					return -1;

				written += len;
				fmt++;
				break;
			}
			// Signed decimal integer
			case 'i':
			case 'd':
			{
				const i32 num = va_arg(args, i32);

				// The largest signed integer is 2^32, which uses
				// 10 digits + NUL.
				char str[10 + 1];
				itoa(num, str, 10);
				const usize len = strlen(str);
				if (!print(str, len))
					return -1;

				written += len;
				fmt++;
				break;
			}
			// Unsigned decimal integer
			case 'u':
			{
				const u32 num = va_arg(args, u32);

				// The largest signed integer is 2^32, which uses
				// 10 digits + NUL.
				char str[10 + 1];
				utoa(num, str, 10);
				const usize len = strlen(str);
				if (!print(str, len))
					return -1;

				written += len;
				fmt++;
				break;
			}
			// Unsigned hexadecimal integer
			case 'x':
			{
				const u32 num = va_arg(args, u32);

				char str[sizeof(u32) * 2 + 1];
				utoa(num, str, 16);

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

				fmt++;
				written += len;
				break;
			}
			// Unsigned hexadecimal integer (uppercase)
			case 'X':
			{
				const u32 num = va_arg(args, u32);

				char str[sizeof(u32) * 2 + 1];
				utoa(num, str, 16);

				const usize len = strlen(str);

				// Print prefix if '#' was previous format.
				if (write_prefix)
					if (!print("0x", 2))
						return -1;
				if (!print(str, len))
					return -1;

				fmt++;
				written += len;
				break;
			}
			// Pointer address
			case 'p':
			{
				const usize num = va_arg(args, usize);
				const usize buf_size = sizeof(usize) * 2 + 1;
				char str[buf_size];
				lutoa(num, str, 0x10);
				const usize len = strlen(str);		  // Get the length of the final number.
				for (int i = 0; i < buf_size; i++)	  // Fill with zeroes.
					str[i] = '0';
				char* offset = str + (sizeof(str) - len - 1);
				lutoa(num, offset, 0x10);	 // Write the number again, but now at an offset.

				if (!print(str, buf_size))
					return -1;

				fmt++;
				written += len;
				break;
			}
			// The number of characters written so far is stored in the pointed location.
			case 'n':
			{
				i32* const ptr = va_arg(args, i32*);
				*ptr = written;
				fmt++;
				break;
			}
			// No format, just normal text.
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
