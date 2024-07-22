//? A global list of all system calls available.

#ifdef SYSCALL_TABLE_INSERT
#define SYSCALL(num, name) [num] = syscall_##name,
#else
#include <menix/common.h>
#define SYSCALL(num, name) size_t syscall_##name(size_t a0, size_t a1, size_t a2, size_t a3, size_t a4, size_t a5);
#endif

SYSCALL(0, null)
// SYSCALL(1, read)
// SYSCALL(2, write)
// SYSCALL(3, open)
// SYSCALL(4, close)

#include <bits/syscall_list.h>
