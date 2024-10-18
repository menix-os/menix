#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/thread/process.h>
#include <menix/thread/scheduler.h>

// Returns the ID of the calling process.
SYSCALL_IMPL(pid)
{
	return arch_current_cpu()->thread->parent->id;
}

// Returns the ID of the parent of the calling process.
SYSCALL_IMPL(pid_parent)
{
	// Get the parent of the current process.
	Process* parent_process = arch_current_cpu()->thread->parent->parent;

	if (parent_process != NULL)
	{
		return parent_process->id;
	}

	return 0;
}
