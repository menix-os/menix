//? Kernel error output

#include <menix/log.h>
#include <stdarg.h>
#include <stdio.h>

void kmesg(int32_t level, const char* fmt, ...)
{
	// TODO: Add timer to output

	va_list args;
	va_start(args, fmt);

	switch (level)
	{
#ifdef DEBUG
		case LOG_DEBUG:
			puts("[DEBUG] ");
			break;
#endif
		case LOG_INFO:
			puts("[INFO ] ");
			break;
		case LOG_ERR:
			puts("[ERROR] ");
			break;
		default:
			puts("[WARN] ");
			break;
	}

	vprintf(fmt, args);
	va_end(args);
}
