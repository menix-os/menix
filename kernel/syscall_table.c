//? System call lookup table.

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/syscall.h>

// Include the syscalls once.
#include <menix/syscall_list.h>

const SyscallFn syscall_table[] = {
// Include them again, but now as table entry.
#undef SYSCALL
#define SYSCALL_TABLE_INSERT
#include <menix/syscall_list.h>
#undef SYSCALL_TABLE_INSERT
};
const size_t syscall_table_size = ARRAY_SIZE(syscall_table);
