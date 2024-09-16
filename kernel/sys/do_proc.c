// Syscall "exec"

#include <menix/sys/syscall.h>
#include <menix/thread/process.h>

#include <errno.h>

// Starts a new process from an ELF executable.
// `path`: The path where the executable to load is stored.
// `argv`: A NULL-terminated list of program arguments to be passed to the new process.
// `envp`: A NULL-terminated list of environment variables to be passed to the new process.
SYSCALL_IMPL(execve, const char* path, char** argv, char** envp)
{
	if (process_execve(path, argv, envp))
		return 0;
	else
		return -ENOEXEC;
}

// Terminates the current process.
// `status`: The status code to return to the parent process.
SYSCALL_IMPL(exit, u8 status)
{
	// TODO:
	kmesg("Terminating with exit code %hhu...\n", status);
	// proc_exit(arch_current_cpu()->thread->parent, status);
	return 0;
}
