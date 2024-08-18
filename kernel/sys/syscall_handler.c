// Invokes a kernel function from a system call.
// syscall_handler is to be invoked by a user process, thus its declaration should be invisible to the kernel.

#include <menix/log.h>
#include <menix/sys/syscall.h>

#include <errno.h>

void syscall_handler(SyscallArgs* regs)
{
	// First argument contains the syscall number.
	// Check if number is inside bounds.
	if (regs->num >= syscall_table_size)
	{
		kmesg("Attempted to execute unrecognized syscall %u\n", regs->num);
		regs->num = -ENOSYS;
		return;
	}

	// Execute the system call.
	// The syscall selector also contains the return value.
	regs->num = syscall_table[regs->num](regs);
}
