#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>

SYSCALL_IMPL(kill, usize pid, usize sig)
{
	Process* process = sch_id_to_process(pid);
	if (process == NULL)
		return SYSCALL_ERR(EINVAL);

	// TODO: process->return_code = SIGKILL;
	proc_kill(process, false);
	return SYSCALL_OK(0);
}
