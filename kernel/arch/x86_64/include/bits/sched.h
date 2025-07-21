#ifndef MENIX_ARCH_SCHED_H
#define MENIX_ARCH_SCHED_H

#include <menix/types.h>

struct arch_task_context {
	u64 rsp;
	u8* fpu_region;
	u16 ds, es, fs, gs;
	u64 fs_base, gs_base;
};

struct arch_context {
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
	u64 isr;   // Pushed onto the stack by the interrupt handler stubs.
	u64 error; // Pushed onto the stack by the CPU if the interrupt has an error code.
	u64 rip;   // The rest is pushed onto the stack by the CPU during an interrupt.
	u64 cs;
	u64 rflags;
	u64 rsp;
	u64 ss;
};

#endif
