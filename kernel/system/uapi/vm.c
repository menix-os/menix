// uAPI vm.h implementation

#include <menix/common.h>
#include <menix/memory/vm.h>
#include <menix/util/log.h>

#include <uapi/status.h>
#include <uapi/types.h>

#define UAPI_KERNEL_API
#define UAPI_WANTS_VM
#include <uapi/vm.h>
#undef UAPI_KERNEL_API

void* uapi_kernel_map(uapi_phys_addr addr, uapi_size len, uapi_caching, uapi_access_type type)
{
	VMProt prot = 0;
	if (type & UAPI_ACCESS_TYPE_READ)
		prot |= VMProt_Read;
	if (type & UAPI_ACCESS_TYPE_WRITE)
		prot |= VMProt_Write;
	if (type & UAPI_ACCESS_TYPE_EXECUTE)
		prot |= VMProt_Execute;

	return vm_map_memory(addr, len, prot);
}

void uapi_kernel_unmap(void* addr, uapi_size len)
{
	// TODO
}
