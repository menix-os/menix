// Kernel error output

#include <menix/common.h>
#include <menix/log.h>
#include <menix/serial.h>

#include <stdarg.h>
#include <stdio.h>

void kmesg(const char* fmt, ...)
{
	va_list args;
	va_start(args, fmt);
	vprintf(fmt, args);
	va_end(args);
}
