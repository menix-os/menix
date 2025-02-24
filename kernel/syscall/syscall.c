// Abstract syscall selector

#include <menix/common.h>
#include <menix/syscall/syscall.h>
#include <menix/util/log.h>

#include <uapi/errno.h>

// Include the syscalls once. Use weak linkage for stubs.
#undef SYSCALL
#define SYSCALL(num, name, ...) [[gnu::weak]] SyscallResult syscall_##name(usize, usize, usize, usize, usize, usize);
#include <uapi/syscall_list.h>
#undef SYSCALL

struct SyscallTable
{
	SyscallFn func;
	const char* func_name;
};

static const struct SyscallTable syscall_table[] = {
// Include them again, but now as table entry.
#define SYSCALL(num, name, ...) [num] = {.func = (SyscallFn)syscall_##name, .func_name = #name},
#include <uapi/syscall_list.h>
#undef SYSCALL
};

SyscallResult syscall_invoke(usize num, usize a0, usize a1, usize a2, usize a3, usize a4, usize a5)
{
	// First argument contains the syscall number.
	// Check if number is inside bounds.
	if (unlikely(num >= ARRAY_SIZE(syscall_table)))
	{
		print_log("User program called syscall %zu, but this is out of bounds!\n", num);
		return SYSCALL_ERR(ENOSYS);
	}

	if (unlikely(syscall_table[num].func == NULL))
	{
		print_log("User program called syscall %zu (\"%s\"), but it is not implemented!\n", num,
				  syscall_table[num].func_name);
		return SYSCALL_ERR(ENOSYS);
	}

	// Execute the system call.
	return syscall_table[num].func(a0, a1, a2, a3, a4, a5);
}
