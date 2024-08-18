// System call lookup table.

#include <menix/common.h>
#include <menix/sys/syscall.h>

// Include the syscalls once.
#include <menix/sys/syscall_list.h>

const SyscallFn syscall_table[] = {
// Include them again, but now as table entry.
#undef SYSCALL
#define SYSCALL_TABLE_INSERT
#include <menix/sys/syscall_list.h>
#undef SYSCALL_TABLE_INSERT
};

const usize syscall_table_size = ARRAY_SIZE(syscall_table);
