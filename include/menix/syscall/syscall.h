// System call interface + prototypes

#pragma once

#include <menix/common.h>
#include <menix/util/log.h>
#include <menix/util/self.h>

// This macro should be used when implementing a syscall, so that the naming scheme is centralized.
#define SYSCALL_IMPL(name, ...) usize syscall_##name(__VA_ARGS__)

// A syscall that is yet to be implemented.
#define SYSCALL_STUB(name, ...) \
	SYSCALL_IMPL(name, __VA_ARGS__) \
	{ \
		kmesg("Call to unimplemented syscall " #name "!\n"); \
		return 0; \
	}

typedef usize (*SyscallFn)(usize, usize, usize, usize, usize, usize);

usize syscall_invoke(usize num, usize a0, usize a1, usize a2, usize a3, usize a4, usize a5);
