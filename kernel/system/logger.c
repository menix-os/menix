#include <menix/common.h>
#include <menix/system/arch.h>
#include <menix/system/elf.h>
#include <menix/system/logger.h>
#include <menix/system/module.h>
#include <menix/system/sch/thread.h>
#include <menix/system/time/clock.h>
#include <menix/util/format.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

#include <stdarg.h>
#include <stdint.h>

static LoggerWriteFn logger_callbacks[32] = {0};

void logger_register(const char* name, LoggerWriteFn callback)
{
	for (usize i = 0; i < ARRAY_SIZE(logger_callbacks); i++)
	{
		if (logger_callbacks[i] == NULL)
		{
			logger_callbacks[i] = callback;
			print_log("log: Registered new logging sink \"%s\"\n", name);
			return;
		}
	}
	// If we get here, no callback slots are free anymore.
	print_warn("log: Unable to register new callback function, all slots are in use!\n");
}

void logger_write(const char* buf, usize len)
{
	for (usize i = 0; i < ARRAY_SIZE(logger_callbacks); i++)
	{
		if (likely(logger_callbacks[i]))
			logger_callbacks[i](buf, len);
	}
}

SpinLock kmesg_lock;

static i32 kprintf_internal(const char* restrict fmt, va_list args)
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
			logger_write(fmt, amount);
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

				logger_write(&c, sizeof(c));

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

				logger_write(str, len);

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
						logger_write(" ", 1);
				}
				if (has_width)
				{
					char c = pad_zero ? '0' : ' ';

					// We don't have to pad anything.
					if (len < width)
					{
						for (usize i = 0; i < width - len; i++)
							logger_write(&c, 1);
					}
				}
				if (write_prefix)
				{
					logger_write("0x", 2);
				}
				if (force_sign)
				{
					char c = '+';
					if (number[0] != '-')
						logger_write(&c, 1);
				}
				if (blank_sign)
				{
					char c = ' ';
					if (number[0] != '-')
						logger_write(&c, 1);
				}
				logger_write(number, len);

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
#if ARCH_BITS == 64
				u64toa(num, number, 0x10);
#else
				u32toa(num, number, 0x10);
#endif
				const usize len = strlen(number);	  // Get the length of the final number.
				for (int i = 0; i < buf_size; i++)	  // Fill with zeroes.
					str[i] = '0';
				usize offset = sizeof(str) - len - 1;

				logger_write(str, offset);
				logger_write(number, len);

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

				logger_write(fmt, len);

				written += len;
				fmt += len;
				break;
			}
		}
	}
	return written;
}

i32 kprintf(const char* restrict fmt, ...)
{
	va_list args;
	va_start(args, fmt);

	i32 written = kprintf_internal(fmt, args);

	va_end(args);
	return written;
}

void kmesg(const char* fmt, ...)
{
	usize __time = clock_get_elapsed_ns();
	usize __secs = (__time / 1000000000);
	usize __millis = ((__time / 1000) % 1000000);
	CpuInfo* __cpu = arch_current_cpu();
	usize __tid = 0;
	if (__cpu != NULL && __cpu->thread != NULL)
		__tid = __cpu->thread->id;
	kprintf("[%5zu.%06zu] [%7zu] ", __secs, __millis, __tid);

	spin_lock(&kmesg_lock);

	va_list args;
	va_start(args, fmt);
	kprintf_internal(fmt, args);
	va_end(args);

	spin_unlock(&kmesg_lock);
}

typedef struct ATTR(packed) StackFrame
{
	struct StackFrame* prev;	// The inner frame.
	void* return_addr;			// The address this frame returns to.
} StackFrame;

void ktrace(Context* regs)
{
	// If no context is given, raise a software interrupt reserved for this purpose.
	if (regs == NULL)
	{
#if defined(__x86_64__)
		// TODO(port): Convert to define, this only works on x86_64
		asm volatile("int $3");
#endif
	}
	else
		arch_dump_registers(regs);

	StackFrame* fp = __builtin_frame_address(0);

	// Print stack trace.
	print_log("--- Stack trace (Most recent call first) ---\n");
	for (usize i = 0; i < 32 && fp != NULL; fp = fp->prev, i++)
	{
		// Try to resolve the symbol name and offset.
		const char* name;
		Elf_Sym* sym;

		// If we have found the corresponding symbol, print its name + offset.
		if (module_find_symbol(fp->return_addr, &name, &sym))
			print_log("\t[%zu]\t0x%p <%s + 0x%zx>\n", i, fp->return_addr, name, fp->return_addr - sym->st_value);

		// If the address is not NULL, but we don't have any matching symbol, just print the address.
		else if (fp->return_addr)
			print_log("\t[%zu]\t0x%p <\?\?\?>\n", i, fp->return_addr);
	}
	print_log("--- End of Stack trace ---\n");
}

ATTR(noreturn) void panic()
{
	print_error("Panic was triggered! Stopping machine.\n");
	arch_stop();
}
