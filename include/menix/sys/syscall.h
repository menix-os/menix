// System call interface + prototypes

#include <menix/common.h>
#include <menix/util/self.h>

#include <bits/syscall.h>

typedef struct
{
	u64 num;
	u64 a0;
	u64 a1;
	u64 a2;
	u64 a3;
	u64 a4;
	u64 a5;
} SyscallArgs;

// This macro should be used when implementing a syscall, so that the naming scheme is centralized.
#define SYSCALL_IMPL(name) usize syscall_##name(usize a0, usize a1, usize a2, usize a3, usize a4, usize a5)

typedef usize (*SyscallFn)(usize a0, usize a1, usize a2, usize a3, usize a4, usize a5);

// Contains all system calls.
extern const SyscallFn syscall_table[];

// Total amount of syscalls.
extern const usize syscall_table_size;
