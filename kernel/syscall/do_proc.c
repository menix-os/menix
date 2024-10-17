// Syscalls for process management

#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/thread/process.h>
#include <menix/thread/scheduler.h>

// Starts a new process from an ELF executable. Returns 0 upon success, otherwise -1.
// `path`: The path where the executable to load is stored.
// `argv`: A NULL-terminated list of program arguments to be passed to the new process.
// `envp`: A NULL-terminated list of environment variables to be passed to the new process.
SYSCALL_IMPL(execve, const char* path, char** argv, char** envp)
{
	if (process_execve(path, argv, envp) == true)
		return 0;
	else
		return -1;
}

// Terminates the current process.
// `status`: The status code to return to the parent process.
SYSCALL_IMPL(exit, u8 status)
{
	Process* process = arch_current_cpu()->thread->parent;
	process->return_code = status;
	process_kill(process, false);
	return 0;
}

// Forcefully terminates a process.
// `pid`: The ID of the process to kill.
SYSCALL_IMPL(kill, usize pid)
{
	Process* process = scheduler_id_to_process(pid);
	if (process == NULL)
		return -1;

	// TODO: process->return_code = SIGKILL;
	process_kill(process, false);
	return 0;
}

// Returns the ID of the calling process.
SYSCALL_IMPL(get_pid)
{
	return arch_current_cpu()->thread->parent->id;
}

// Returns the ID of the parent of the calling process.
SYSCALL_IMPL(get_parent_pid)
{
	// Get the parent of the current process.
	Process* parent_process = arch_current_cpu()->thread->parent->parent;

	if (parent_process != NULL)
	{
		return parent_process->id;
	}

	return 0;
}

// Forks a thread by cloning its attributes.
SYSCALL_IMPL(fork)
{
	Thread* thread = arch_current_cpu()->thread;
	return process_fork(thread->parent, thread);
}
