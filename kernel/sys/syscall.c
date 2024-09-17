// Abstract syscall selector

#include <menix/sys/syscall.h>
#include <menix/util/log.h>

#include <errno.h>

usize syscall_invoke(usize num, usize a0, usize a1, usize a2, usize a3, usize a4, usize a5)
{
	// First argument contains the syscall number.
	// Check if number is inside bounds.
	if (num >= syscall_table_size)
	{
		kmesg("Attempted to execute unrecognized syscall %u\n", num);
		return -ENOSYS;
	}

	// Execute the system call.
	return syscall_table[num](a0, a1, a2, a3, a4, a5);
}
