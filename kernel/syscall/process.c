// Syscalls for process management

#include <menix/abi/errno.h>
#include <menix/fs/vfs.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>

#include "menix/system/abi.h"

// Forks a thread by cloning its attributes.
SYSCALL_IMPL(fork)
{
	Thread* thread = arch_current_cpu()->thread;
	return SYSCALL_OK(proc_fork(thread->parent, thread));
}

// Terminates the current process.
// `status`: The status code to return to the parent process.
SYSCALL_IMPL(exit, u8 status)
{
	Process* process = arch_current_cpu()->thread->parent;
	process->return_code = status;
	proc_kill(process, false);
	return SYSCALL_OK(0);
}

// Forcefully terminates a process.
// `pid`: The ID of the process to kill.
// `sig`: The signal to send to the process.
SYSCALL_IMPL(kill, usize pid, usize sig)
{
	Process* process = sch_id_to_process(pid);
	if (process == NULL)
		return SYSCALL_ERR(EINVAL);

	// TODO: process->return_code = SIGKILL;
	proc_kill(process, false);
	return SYSCALL_OK(0);
}

// Starts a new process from an ELF executable. Returns 0 upon success, otherwise -1.
// `path`: The path where the executable to load is stored.
// `argv`: A NULL-terminated list of program arguments to be passed to the new process.
// `envp`: A NULL-terminated list of environment variables to be passed to the new process.
SYSCALL_IMPL(execve, const char* path, char** argv, char** envp)
{
	if (path == NULL)
		return SYSCALL_ERR(ENOENT);

	vm_user_access({
		if (proc_execve(NULL, path, argv, envp, true) == true)
			return SYSCALL_OK(0);
		else
			return SYSCALL_ERR(arch_current_cpu()->thread->errno);
	});
}

// Returns the ID of the calling process.
SYSCALL_IMPL(getpid)
{
	return SYSCALL_OK(arch_current_cpu()->thread->parent->id);
}

// Returns the ID of the parent of the calling process.
SYSCALL_IMPL(getparentpid)
{
	// Get the parent of the current process.
	Process* parent_process = arch_current_cpu()->thread->parent->parent;

	if (parent_process != NULL)
		return SYSCALL_OK(parent_process->id);

	return SYSCALL_OK(0);
}

SYSCALL_IMPL(getcwd, char* buf, usize size)
{
	if (buf == NULL || size == 0 || size > PATH_MAX)
		return SYSCALL_ERR(ERANGE);

	Process* proc = arch_current_cpu()->thread->parent;
	usize written;
	vm_user_access({ written = vfs_get_path(proc->working_dir, buf, size); });

	if (written != size)
		return SYSCALL_ERR(ERANGE);

	return SYSCALL_OK(0);
}

SYSCALL_STUB(setuid)
SYSCALL_STUB(getuid)

SYSCALL_STUB(setgid)
SYSCALL_STUB(getgid)
