#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/sch/thread.h>

#include <uapi/errno.h>

SYSCALL_IMPL(execve, const char* path, char** argv, char** envp)
{
	if (path == NULL)
		return SYSCALL_ERR(ENOENT);

	if (proc_create_elf(NULL, path, argv, envp, true) == true)
		return SYSCALL_OK(0);
	else
		return SYSCALL_ERR(arch_current_cpu()->thread->errno);
}
