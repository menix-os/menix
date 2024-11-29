// System call interface + prototypes

#pragma once

#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/util/log.h>
#include <menix/util/self.h>

typedef struct SyscallResult
{
	usize value;
	usize error;
} SyscallResult;

#define SYSCALL_OK(val) \
	(SyscallResult) \
	{ \
		.value = (usize)(val), .error = 0 \
	}

#define SYSCALL_ERR(err) \
	(SyscallResult) \
	{ \
		.value = 0, .error = (usize)(err) \
	}

#define SYSCALL_FAIL(val, err) \
	(SyscallResult) \
	{ \
		.value = (usize)(val), .error = (usize)(err) \
	}

// This macro should be used when implementing a syscall, so that the naming scheme is centralized.
#define SYSCALL_IMPL(name, ...) SyscallResult syscall_##name(__VA_ARGS__)

// A syscall that is yet to be implemented.
#define SYSCALL_STUB(name, ...) \
	SYSCALL_IMPL(name, __VA_ARGS__) \
	{ \
		print_log("Call to unimplemented syscall " #name "!\n"); \
		return SYSCALL_ERR(ENOSYS); \
	}

typedef SyscallResult (*SyscallFn)(usize, usize, usize, usize, usize, usize);

SyscallResult syscall_invoke(usize num, usize a0, usize a1, usize a2, usize a3, usize a4, usize a5);
