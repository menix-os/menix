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

	// Pushed onto the stack by the CPU if exception has an error code.
	u64 error;
	// Pushed onto the stack by the CPU during an interrupt.
	u64 rip;
	u64 cs;
	u64 rflags;
	u64 rsp;
	u64 ss;
} Context;

typedef struct
{
	u16 fcw;
	u16 fsw;
	u8 ftw;
	u8 reserved0;
	u16 fop;
	u64 fpuIp;
	u64 fpuDp;
	u32 mxcsr;
	u32 mxcsrMask;
	u8 st0[10];
	u8 reserved1[6];
	u8 st1[10];
	u8 reserved2[6];
	u8 st2[10];
	u8 reserved3[6];
	u8 st3[10];
	u8 reserved4[6];
	u8 st4[10];
	u8 reserved5[6];
	u8 st5[10];
	u8 reserved6[6];
	u8 st6[10];
	u8 reserved7[6];
	u8 st7[10];
	u8 reserved8[6];
	u8 xmm0[16];
	u8 xmm1[16];
	u8 xmm2[16];
	u8 xmm3[16];
	u8 xmm4[16];
	u8 xmm5[16];
	u8 xmm6[16];
	u8 xmm7[16];
	u8 xmm8[16];
	u8 xmm9[16];
	u8 xmm10[16];
	u8 xmm11[16];
	u8 xmm12[16];
	u8 xmm13[16];
	u8 xmm14[16];
	u8 xmm15[16];
	u8 reserved9[48];
	u8 available[48];
} FxState;
