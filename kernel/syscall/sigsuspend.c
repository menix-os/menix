// Syscalls for process management

#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/sch/thread.h>

SYSCALL_IMPL(sigsuspend)
{
	// TODO: sigmask
	Thread* thread = arch_current_cpu()->thread;
	thread->state = ThreadState_Sleeping;
	while (thread->state == ThreadState_Sleeping)
	{
		sch_arch_invoke();
	}
	return SYSCALL_ERR(EINTR);
}
