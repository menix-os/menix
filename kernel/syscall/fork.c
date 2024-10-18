// Syscalls for process management

#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/thread/process.h>
#include <menix/thread/scheduler.h>

// Forks a thread by cloning its attributes.
SYSCALL_IMPL(fork)
{
	Thread* thread = arch_current_cpu()->thread;
	return process_fork(thread->parent, thread);
}
