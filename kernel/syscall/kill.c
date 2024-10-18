#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/thread/process.h>
#include <menix/thread/scheduler.h>

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
