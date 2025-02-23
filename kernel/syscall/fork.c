#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/thread.h>

SYSCALL_IMPL(fork)
{
	Thread* thread = arch_current_cpu()->thread;
	return SYSCALL_OK(proc_fork(thread->parent, thread));
}
