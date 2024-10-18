#include <menix/common.h>
#include <menix/syscall/syscall.h>
#include <menix/thread/process.h>

// Terminates the current process.
// `status`: The status code to return to the parent process.
SYSCALL_IMPL(exit, u8 status)
{
	Process* process = arch_current_cpu()->thread->parent;
	process->return_code = status;
	process_kill(process, false);
	return 0;
}
