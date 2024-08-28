// A global list of all system calls available.

#ifdef SYSCALL_TABLE_INSERT
#define SYSCALL(num, name) [num] = (SyscallFn)syscall_##name,
#else
#include <menix/common.h>
#define SYSCALL(num, name) usize syscall_##name(usize a0, usize a1, usize a2, usize a3, usize a4, usize a5);
#endif

SYSCALL(0, exit)
SYSCALL(1, write)
SYSCALL(2, read)
SYSCALL(3, ioctl)
SYSCALL(4, execve)
SYSCALL(32, uname)

#include <bits/syscall_list.h>
