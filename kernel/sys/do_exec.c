// Syscall "exec"

#include <menix/sys/syscall.h>
#include <menix/thread/process.h>

#include <errno.h>

// Starts a new process from an ELF executable.
// `path`: The path where the executable to load is stored.
// `argv`: A NULL-terminated list of program arguments to be passed to the new process.
// `envp`: A NULL-terminated list of environment variables to be passed to the new process.
SYSCALL_IMPL(exec)
{
	const char* path = (const char*)args->a0;
	char** argv = (char**)args->a1;
	char** envp = (char**)args->a2;

	if (process_execute(path, argv, envp))
		return 0;
	else
		return -ENOEXEC;
}
