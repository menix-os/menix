// uAPI malloc.h implementation

#include <menix/common.h>
#include <menix/util/log.h>

#include <uapi/status.h>
#include <uapi/types.h>

#define UAPI_KERNEL_API
#define UAPI_WANTS_MALLOC
#include <uapi/malloc.h>
#undef UAPI_KERNEL_API

void* uapi_kernel_alloc(uapi_size size)
{
	return kmalloc(size);
}

void* uapi_kernel_calloc(uapi_size count, uapi_size size)
{
	return kzalloc(count * size);
}

void uapi_kernel_free(void* mem)
{
	kfree(mem);
}
