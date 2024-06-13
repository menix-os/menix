/*-----------------------------------------
Kernel C library - "stdio.h" implementation
-----------------------------------------*/

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

int printf(const char* restrict format, ...)
{
	va_list parameters;
	va_start(parameters, format);

	// Amount of bytes written.
	int written = 0;

	while (*format != '\0')
	{
		size_t maxrem = INT32_MAX - written;

		if (format[0] != '%' || format[1] == '%')
		{
			if (format[0] == '%')
				format++;
			size_t amount = 1;
			while (format[amount] && format[amount] != '%')
				amount++;
			if (maxrem < amount)
			{
				// TODO: Set errno to EOVERFLOW.
				return -1;
			}
			if (!print(format, amount))
				return -1;
			format += amount;
			written += amount;
			continue;
		}

		const char* format_begun_at = format++;

		switch (*format)
		{
				// Character
			case 'c':
			{
				format++;
				char c = (char)va_arg(parameters, int32_t);

				if (!print(&c, sizeof(c)))
					return -1;

				written++;
				break;
			}
			// String of characters
			case 's':
			{
				format++;
				const char* str = va_arg(parameters, const char*);

				size_t len = strlen(str);
				if (!print(str, len))
					return -1;

				written += len;
				break;
			}
			case 'i':
			case 'd':
			{
				format++;
				const int32_t num = va_arg(parameters, int32_t);

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
				format++;
				const uint32_t num = va_arg(parameters, uint32_t);

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
				format++;
				const uint32_t num = va_arg(parameters, uint32_t);

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
				format++;
				const uintptr_t num = va_arg(parameters, uintptr_t);

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
				format = format_begun_at;
				size_t len = strlen(format);

				if (!print(format, len))
					return -1;

				written += len;
				format += len;
				break;
			}
		}
	}

	va_end(parameters);
	return written;
}
