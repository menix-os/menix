#pragma once
#include <menix/common.h>

typedef struct Context
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

	// Pushed onto the stack by the interrupt handler stub.
	u64 isr;
	// Pushed onto the stack by the CPU if exception has an error code.
	u64 error;
	// Pushed onto the stack by the CPU during an interrupt.
	u64 rip;
	u64 cs;
	u64 rflags;
	u64 rsp;
	u64 ss;
} Context;
