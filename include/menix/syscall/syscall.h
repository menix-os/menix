// System call interface + prototypes

#include <menix/common.h>
#include <menix/util/self.h>

// This macro should be used when implementing a syscall, so that the naming scheme is centralized.
#define SYSCALL_IMPL(name, ...) usize syscall_##name(__VA_ARGS__)

typedef usize (*SyscallFn)(usize, usize, usize, usize, usize, usize);

usize syscall_invoke(usize num, usize a0, usize a1, usize a2, usize a3, usize a4, usize a5);

// Contains all system calls.
extern const SyscallFn syscall_table[];

// Total amount of syscalls.
extern const usize syscall_table_size;
