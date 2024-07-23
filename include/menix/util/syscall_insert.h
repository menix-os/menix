// Allows function declaration + table insertion

#ifdef SYSCALL_TABLE_INSERT
#define SYSCALL(num, name) [num] = (SyscallFn)syscall_##name,
#else
#include <menix/common.h>
#define SYSCALL(num, name) size_t syscall_##name(size_t a0, size_t a1, size_t a2, size_t a3, size_t a4, size_t a5);
#endif
