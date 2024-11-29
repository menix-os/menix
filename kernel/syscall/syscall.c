// Abstract syscall selector

#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/syscall/syscall.h>
#include <menix/util/log.h>

// Include the syscalls once.
#include <menix/syscall/syscall_list.h>

static const SyscallFn syscall_table[] = {
// Include them again, but now as table entry.
#undef SYSCALL
#define SYSCALL_TABLE_INSERT
#include <menix/syscall/syscall_list.h>
#undef SYSCALL_TABLE_INSERT
};

SyscallResult syscall_invoke(usize num, usize a0, usize a1, usize a2, usize a3, usize a4, usize a5)
{
	// First argument contains the syscall number.
	// Check if number is inside bounds.
	if (num >= ARRAY_SIZE(syscall_table))
	{
		print_log("Attempted to execute unrecognized syscall %u\n", num);
		return SYSCALL_ERR(ENOSYS);
	}

	if (syscall_table[num] == NULL)
		return SYSCALL_ERR(ENOSYS);

	// Execute the system call.
	return syscall_table[num](a0, a1, a2, a3, a4, a5);
}
