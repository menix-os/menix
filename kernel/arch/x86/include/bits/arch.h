// x86 specific declarations.

#pragma once

#include <menix/common.h>
#include <menix/thread/process.h>

#if CONFIG_page_size != 4096
#error "Page size must be exactly 4KiB!"
#endif

#define MSR_EFER   0xC0000080
#define MSR_STAR   0xC0000081
#define MSR_LSTAR  0xC0000082
#define MSR_CSTAR  0xC0000083
#define MSR_SFMASK 0xC0000084

struct ArchCpu
{
	u64 id;				 // Unique index of this CPU.
	Thread* thread;		 // Current thread running on this CPU.
	u64 kernel_stack;	 // RSP for the kernel.
	u64 user_stack;		 // RSP for the user space.
};

// All code-visible CPU registers.
struct ArchRegisters
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
};
