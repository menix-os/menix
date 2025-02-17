// Abstract syscall selector

#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/syscall/syscall.h>
#include <menix/util/log.h>

// Include the syscalls once.
#undef SYSCALL
#define SYSCALL(num, name) SyscallResult syscall_##name(usize a0, usize a1, usize a2, usize a3, usize a4, usize a5);
#include <menix/syscall/syscall_list.h>

typedef struct
{
	SyscallFn func;
	const char* func_name;
} SyscallTable;

static const SyscallTable syscall_table[] = {
// Include them again, but now as table entry.
#undef SYSCALL
#define SYSCALL(num, name) [num] = {.func = (SyscallFn)syscall_##name, .func_name = #name},
#include <menix/syscall/syscall_list.h>
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

	if (syscall_table[num].func == NULL)
		return SYSCALL_ERR(ENOSYS);

	// Execute the system call.
	return syscall_table[num].func(a0, a1, a2, a3, a4, a5);
}
