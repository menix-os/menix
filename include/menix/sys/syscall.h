// System call interface + prototypes

#include <menix/common.h>
#include <menix/util/self.h>

#include <bits/syscall.h>

typedef struct
{
	u64 num;	// The system call selector.
	u64 a0;		// Argument 0.
	u64 a1;		// Argument 1.
	u64 a2;		// Argument 2.
	u64 a3;		// Argument 3.
	u64 a4;		// Argument 4.
	u64 a5;		// Argument 5.
} SyscallArgs;

// This macro should be used when implementing a syscall, so that the naming scheme is centralized.
#define SYSCALL_IMPL(name) usize syscall_##name(SyscallArgs* args)

typedef usize (*SyscallFn)(SyscallArgs* args);

// Contains all system calls.
extern const SyscallFn syscall_table[];

// Total amount of syscalls.
extern const usize syscall_table_size;
