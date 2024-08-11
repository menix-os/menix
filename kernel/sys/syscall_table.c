// System call lookup table.

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/syscall.h>

#include <errno.h>

// Include the syscalls once.
#include <menix/syscall_list.h>

const SyscallFn syscall_table[] = {
// Include them again, but now as table entry.
#undef SYSCALL
#define SYSCALL_TABLE_INSERT
#include <menix/syscall_list.h>
#undef SYSCALL_TABLE_INSERT
};

const usize syscall_table_size = ARRAY_SIZE(syscall_table);

void syscall_handler(SyscallArgs* regs)
{
	// First argument contains the syscall number.
	// Check if number is inside bounds.
	if (regs->num >= syscall_table_size)
	{
		regs->num = -ENOSYS;
		return;
	}
	SyscallFn fn = syscall_table[regs->num];
	regs->num = fn(regs->a0, regs->a1, regs->a2, regs->a3, regs->a4, regs->a5);
}
