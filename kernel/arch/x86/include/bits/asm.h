// x86 specific inline assembly snippets.

#pragma once

#include <menix/common.h>

#define asm_gdt_set(table)		 asm volatile("lgdt %0" ::"m"(table))
#define asm_interrupt_disable()	 asm volatile("cli")
#define asm_interrupt_enable()	 asm volatile("sti")
#define asm_get_frame_pointer(x) asm volatile("mov %%rbp, %0" : "=m"(x))
#define asm_pause()				 asm volatile("pause")
#define asm_nop()				 asm volatile("nop")

// Flushes all segment registers and reloads them.
#define asm_flush_segment_regs(code_seg, data_seg) \
	asm volatile("push %0\n" \
				 "movq $L_reload_cs, %%rax\n" \
				 "push %%rax\n" \
				 "lretq\n" \
				 "L_reload_cs:\n" \
				 "mov %1, %%ax\n" \
				 "mov %%ax, %%ds\n" \
				 "mov %%ax, %%es\n" \
				 "mov %%ax, %%fs\n" \
				 "mov %%ax, %%gs\n" \
				 "mov %%ax, %%ss\n" \
				 : \
				 : "i"(code_seg), "i"(data_seg) \
				 : "rax")

// Reads a 64-bit value from a given MSR.
static inline u64 asm_rdmsr(u32 msr)
{
	u32 eax;
	u32 edx;

	asm volatile("rdmsr" : "=a"(eax), "=d"(edx) : "c"(msr) : "memory");
	return ((u64)edx << 32) | eax;
}

// Writes a 64-bit value to a given MSR.
static inline void asm_wrmsr(u32 msr, u64 val)
{
	u32 eax = (u32)val;
	u32 edx = val >> 32;

	asm volatile("wrmsr" : : "a"(eax), "d"(edx), "c"(msr) : "memory");
}
