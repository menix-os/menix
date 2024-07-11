//? System call lookup table.

#include <menix/arch.h>
#include <menix/log.h>
#include <menix/syscall.h>

// Include Syscalls as declarations once.
#include <menix/syscalls.h>

const SyscallFn syscall_table[] = {
// Include them again, but now as table entry.
#undef SYSCALL
#define SYSCALL_TABLE_INSERT
#include <menix/syscalls.h>
#undef SYSCALL_TABLE_INSERT
};
