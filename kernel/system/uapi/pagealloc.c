// uAPI pagealloc.h implementation

#include <menix/common.h>
#include <menix/util/log.h>

#include <uapi/status.h>
#include <uapi/types.h>

#define UAPI_KERNEL_API
#define UAPI_WANTS_PAGEALLOC
#define UAPI_SIZED_FREES
#include <uapi/pagealloc.h>
#undef UAPI_KERNEL_API

uapi_phys_addr uapi_kernel_allocate_pages(uapi_size count, uapi_size align, uapi_phys_addr max_phys_addr)
{
	return pm_alloc(count);
}

void uapi_kernel_free_pages(uapi_phys_addr addr, uapi_size count_hint)
{
	pm_free(addr, count_hint);
}
