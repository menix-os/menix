// Kernel C library - "stdio.h" implementation

#include <menix/fs/vfs.h>
#include <menix/io/terminal.h>

#include <limits.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef bool (*PrintFn)(const char* buf, usize length);

// Prints to somewhere using a callback.
static i32 printf_internal(PrintFn print, const char* restrict fmt, va_list args)
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
		bool force_sign = false;
		bool blank_sign = false;
		bool write_prefix = false;
		bool pad_zero = false;
		bool has_width = false;
		usize width = 0;
		bool has_precision = false;
		usize precision = 0;
		usize size = sizeof(u32);
		char number[64];
check_fmt:
		memset(number, 0, sizeof(number));

		switch (*fmt)
		{
			// Flags
			case '+':
			{
				force_sign = true;
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
				pad_zero = true;
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
				width = atou32(number, 10);

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
					precision = atou32(number, 10);
				}
				goto check_fmt;
			}
			// Length
			case 'h':
			{
				if (fmt[1] == 'h')
				{
					size = sizeof(u8);
					fmt++;
				}
				else
					size = sizeof(u16);
				fmt++;
				goto check_fmt;
			}
			case 'l':
			{
				// There is no difference between `long int` and `long long int`.
				if (fmt[1] == 'l')
					fmt++;
				fmt++;
				size = sizeof(u64);

				goto check_fmt;
			}
			case 'z':
			{
				size = sizeof(usize);
				fmt++;
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
					len = strnlen(str, width);
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
				switch (size)
				{
					case sizeof(i8): i8toa((i8)va_arg(args, i32), number, 10); break;
					case sizeof(i16): i16toa((i16)va_arg(args, i32), number, 10); break;
					case sizeof(i32): i32toa(va_arg(args, i32), number, 10); break;
					case sizeof(i64): i64toa(va_arg(args, i64), number, 10); break;
				}
				goto print_num;
			}
			// Unsigned decimal integer
			case 'u':
			{
				switch (size)
				{
					case sizeof(u8): u8toa((u8)va_arg(args, u32), number, 10); break;
					case sizeof(u16): u16toa((u16)va_arg(args, u32), number, 10); break;
					case sizeof(u32): u32toa(va_arg(args, u32), number, 10); break;
					case sizeof(u64): u64toa(va_arg(args, u64), number, 10); break;
				}
				goto print_num;
			}
			// Unsigned hexadecimal integer
			case 'X':
			case 'x':
			{
				switch (size)
				{
					case sizeof(u8): u8toa((u8)va_arg(args, u32), number, 16); break;
					case sizeof(u16): u16toa((u16)va_arg(args, u32), number, 16); break;
					case sizeof(u32): u32toa(va_arg(args, u32), number, 16); break;
					case sizeof(u64): u64toa(va_arg(args, u64), number, 16); break;
				}
print_num:
				usize len = strlen(number);
				if (has_precision)
				{
					for (usize i = 0; i < precision - len; i++)
						if (!print(" ", 1))
							return -1;
				}
				if (has_width)
				{
					char c = pad_zero ? '0' : ' ';

					// We don't have to pad anything.
					if (len < width)
					{
						for (usize i = 0; i < width - len; i++)
							if (!print(&c, 1))
								return -1;
					}
				}
				if (write_prefix)
				{
					print("0x", 2);
				}
				if (force_sign)
				{
					char c = '+';
					if (number[0] != '-')
						if (!print(&c, 1))
							return -1;
				}
				if (blank_sign)
				{
					char c = ' ';
					if (number[0] != '-')
						if (!print(&c, 1))
							return -1;
				}
				if (!print(number, len))
					return -1;

				written += len;
				fmt++;
				break;
			}
			// Pointer address
			case 'p':
			{
				const usize num = va_arg(args, usize);
				const usize buf_size = sizeof(usize) * 2 + 1;
				char str[buf_size];
#if CONFIG_bits == 64
				u64toa(num, number, 0x10);
#else
				u32toa(num, number, 0x10);
#endif
				const usize len = strlen(number);	  // Get the length of the final number.
				for (int i = 0; i < buf_size; i++)	  // Fill with zeroes.
					str[i] = '0';
				usize offset = sizeof(str) - len - 1;

				if (!print(str, offset))
					return -1;
				if (!print(number, len))
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

static bool print_to_terminal(const char* data, usize length)
{
	terminal_puts(0, data, length);

	// VfsNode* root = vfs_get_root();
	// if (root != NULL)
	//{
	//	VfsNode* node = vfs_get_node(root, "/dev/kmesg", true);
	//	if (node && node->handle)
	//		node->handle->write(node->handle, NULL, data, length, 0);
	// }
	return true;
}

i32 vprintf(const char* restrict fmt, va_list args)
{
	return printf_internal(print_to_terminal, fmt, args);
}

i32 printf(const char* restrict fmt, ...)
{
	va_list args;
	va_start(args, fmt);

	i32 written = vprintf(fmt, args);

	va_end(args);
	return written;
}
