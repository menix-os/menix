// x86 specific syscall declarations.

#pragma once

#include <menix/common.h>

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
