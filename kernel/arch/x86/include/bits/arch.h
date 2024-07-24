// x86 specific declarations.

#pragma once

#include <menix/common.h>

typedef struct
{
	u64 r15;
	u64 r14;
	u64 r13;
	u64 r12;
	u64 r11;
	u64 r10;
	u64 r9;
	u64 r8;
	u64 rsi;
	u64 rdi;
	u64 rbp;
	u64 rdx;
	u64 rcx;
	u64 rbx;
	u64 rax;
	u64 core;
	u64 isr;
	u64 error;
	u64 rip;
	u64 cs;
	u64 rflags;
	u64 rsp;
	u64 ss;
} CpuRegisters;
