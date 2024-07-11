//? Kernel error output

#include <menix/common.h>
#include <menix/log.h>
#include <menix/serial.h>

#include <stdarg.h>
#include <stdio.h>

void kmesg(int32_t level, const char* fmt, ...)
{
	// TODO: Add timer to output

	va_list args;
	va_start(args, fmt);

	switch (level)
	{
#ifndef NDEBUG
		case LOG_DEBUG:
			puts(ANSI_RESET "[" ANSI_COLOR(ANSI_MAGENTA) "DEBUG" ANSI_RESET "] ");
			break;
#endif
		case LOG_INFO:
			puts(ANSI_RESET "[" ANSI_COLOR(ANSI_BLUE) "INFO " ANSI_RESET "] ");
			break;
		case LOG_ERR:
			puts(ANSI_RESET "[" ANSI_COLOR(ANSI_RED) "ERROR" ANSI_RESET "] ");
			break;
		default:
			puts(ANSI_RESET "[" ANSI_COLOR(ANSI_YELLOW) "WARN " ANSI_RESET "] ");
			break;
	}

	vprintf(fmt, args);
	va_end(args);
}
