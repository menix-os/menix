
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
