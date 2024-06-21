//? Kernel error output

#include <menix/log.h>
#include <menix/stdio.h>

#include <stdarg.h>

void klog(int32_t level, const char* fmt, ...)
{
	va_list args;
	va_start(args, format);

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

void kerror(const char* str, ...)
{
	// If we have a message, print it.
	// Otherwise, we don't know.
	printf("[ERROR]\t%s\n", str ? str : "Unknown error!");
}
