#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/syscall/syscall.h>
#include <menix/thread/process.h>

SYSCALL_IMPL(ioctl, u32 fd, u32 request, void* argument)
{
	// TODO
	return 0;
}
