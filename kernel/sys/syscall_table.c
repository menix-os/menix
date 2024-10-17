// System call lookup table.

#include <menix/common.h>
#include <menix/syscall/syscall.h>

// Include the syscalls once.
#include <menix/syscall/syscall_list.h>

const SyscallFn syscall_table[] = {
// Include them again, but now as table entry.
#undef SYSCALL
#define SYSCALL_TABLE_INSERT
#include <menix/syscall/syscall_list.h>
#undef SYSCALL_TABLE_INSERT
};

const usize syscall_table_size = ARRAY_SIZE(syscall_table);
