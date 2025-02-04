// A global list of all system calls available.

#ifdef SYSCALL_TABLE_INSERT
#define SYSCALL(num, name) [num] = {.func = (SyscallFn)name, .func_name = #name},
#else
#include <menix/common.h>
typedef struct SyscallResult SyscallResult;
#define SYSCALL(num, name) SyscallResult name(usize a0, usize a1, usize a2, usize a3, usize a4, usize a5);
#endif
