//? Kernel C library - "stdio.h" implementation

#include <menix/serial.h>
#include <menix/stdint.h>
#include <menix/stdio.h>
#include <menix/stdlib.h>
#include <menix/string.h>

#include <stdarg.h>	   // TODO: Port header.

static bool print(const char* data, size_t length)
{
	const uint8_t* bytes = (const uint8_t*)data;
	for (size_t i = 0; i < length; i++)
		if (putchar(bytes[i]) == EOF)
			return false;
	return true;
}

int32_t putchar(int32_t ic)
{
	char c = (char)ic;
	serial_write(&c, sizeof(c));
	return ic;
}

int32_t puts(const char* str)
{
	const size_t written = strlen(str);
	serial_write(str, written);
	return written;
}

int32_t vprintf(const char* restrict fmt, va_list args)
{
	// Amount of bytes written.
	int written = 0;

	while (*fmt != '\0')
	{
		size_t maxrem = INT32_MAX - written;

		if (fmt[0] != '%' || fmt[1] == '%')
		{
			if (fmt[0] == '%')
				fmt++;
			size_t amount = 1;
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

		switch (*fmt)
		{
				// Character
			case 'c':
			{
				fmt++;
				char c = (char)va_arg(args, int32_t);

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

				size_t len = strlen(str);
				if (!print(str, len))
					return -1;

				written += len;
				break;
			}
			case 'i':
			case 'd':
			{
				fmt++;
				const int32_t num = va_arg(args, int32_t);

				// The largest signed integer is 2^32, which uses
				// 10 digits + NUL.
				char str[10 + 1];
				itoa(num, str, 10);
				const size_t len = strlen(str);
				if (!print(str, len))
					return -1;

				written += len;
				break;
			}
			case 'u':
			{
				fmt++;
				const uint32_t num = va_arg(args, uint32_t);

				// The largest signed integer is 2^32, which uses
				// 10 digits + NUL.
				char str[10 + 1];
				utoa(num, str, 10);
				const size_t len = strlen(str);
				if (!print(str, len))
					return -1;

				written += len;
				break;
			}
			case 'x':
			{
				fmt++;
				const uint32_t num = va_arg(args, uint32_t);

				char str[sizeof(uint32_t) * 2 + 1];
				itoa(num, str, 16);
				const size_t len = strlen(str);
				if (!print(str, len))
					return -1;

				written += len;
				break;
			}
			case 'p':
			{
				fmt++;
				const uintptr_t num = va_arg(args, uintptr_t);

				// Get the hex value, but with all other bytes explicitly
				// written out.
				const size_t buf_size = sizeof(uintptr_t) * 2 + 1;
				char		 str[buf_size];
				itoa(num, str, 16);
				const size_t len = strlen(str);
				for (int i = len; i < buf_size; i++)
					str[i] = '0';
				if (!print(str, buf_size))
					return -1;

				written += len;
				break;
			}
			default:
			{
				fmt = format_begun_at;
				size_t len = strlen(fmt);

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

int32_t printf(const char* restrict fmt, ...)
{
	va_list args;
	va_start(args, fmt);

	int32_t written = vprintf(fmt, args);

	va_end(args);
	return written;
}
