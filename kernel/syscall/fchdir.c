#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>

SYSCALL_IMPL(ioctl, int fd, usize request, usize argument)
{
	// TODO
	return SYSCALL_ERR(ENOSYS);
}
