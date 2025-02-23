#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>

#include <uapi/errno.h>

SYSCALL_IMPL(exit, int status)
{
	Process* process = arch_current_cpu()->thread->parent;
	process->return_code = status;
	proc_kill(process, false);
	return SYSCALL_ERR(EFAULT);
}
