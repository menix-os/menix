#include <menix/fs/fd.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/thread.h>

SYSCALL_IMPL(close, int fd)
{
	Process* process = arch_current_cpu()->thread->parent;

	if (fd_close(process, fd) == false)
		return SYSCALL_ERR(EBADF);

	return SYSCALL_OK(0);
}
