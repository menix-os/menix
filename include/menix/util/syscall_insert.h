// Allows function declaration + table insertion

#ifdef SYSCALL_TABLE_INSERT
#define SYSCALL(num, name) [num] = (SyscallFn)syscall_##name,
#else
#include <menix/common.h>
#define SYSCALL(num, name) usize syscall_##name(usize a0, usize a1, usize a2, usize a3, usize a4, usize a5);
#endif
