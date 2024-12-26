// uAPI log.h implementation

#include <menix/common.h>
#include <menix/util/log.h>

#include <uapi/status.h>
#include <uapi/types.h>

#define UAPI_KERNEL_API
#define UAPI_WANTS_LOG
#include <uapi/log.h>
#undef UAPI_KERNEL_API

void uapi_kernel_log(uapi_log_level level, const uapi_char* msg)
{
	switch (level)
	{
		case UAPI_LOG_INFO:
		case UAPI_LOG_TRACE:
		case UAPI_LOG_DEBUG: print_log("uapi: %s", msg); break;
		case UAPI_LOG_WARN: print_warn("uapi: %s", msg); break;
		case UAPI_LOG_ERROR: print_error("uapi: %s", msg); break;
	}
}
