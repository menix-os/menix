// x86 specific inline assembly snippets.

#pragma once

#include <menix/common.h>

#define asm_set_register(value, reg)  asm volatile("mov %0, %%" #reg ::"r"(value) : "memory")
#define asm_get_register(result, reg) asm volatile("mov %%" #reg ", %0" : "=r"(result)::"memory")
#define asm_gdt_set(table)			  asm volatile("lgdt %0" ::"m"(table))
#define asm_interrupt_disable()		  asm volatile("cli")
#define asm_interrupt_enable()		  asm volatile("sti")
#define asm_get_frame_pointer(x)	  asm volatile("mov %%rbp, %0" : "=m"(x))
#define asm_pause()					  asm volatile("pause")
#define asm_nop()					  asm volatile("nop")
#define asm_swapgs()				  asm volatile("swapgs" ::: "memory")
#define asm_int(int_num)			  asm volatile("int %0" ::"i"(int_num))
#define asm_halt()					  asm volatile("hlt")

// A purposefully invalid instruction for debugging #UD faults. This should never be used in release builds.
#ifdef NDEBUG
#define asm_ill() static_assert(false, "Remove this asm_ill()!")
#else
#define asm_ill() asm("mov %rax, %cr6")
#endif

#define asm_cpuid(leaf, subleaf, a, b, c, d) \
	asm volatile("cpuid" : "=a"(a), "=b"(b), "=c"(c), "=d"(d) : "0"(leaf), "2"(subleaf))

// Flushes all segment registers.
#define asm_flush_segment_regs(code_seg, data_seg) \
	asm volatile("push %0\n" \
				 "leaq L_reload_regs%=(%%rip), %%rax\n" \
				 "push %%rax\n" \
				 "lretq\n" \
				 "L_reload_regs%=:\n" \
				 "mov %1, %%ax\n" \
				 "mov %%ax, %%ss\n" \
				 "xor %%ax, %%ax\n" \
				 "mov %%ax, %%ds\n" \
				 "mov %%ax, %%es\n" \
				 "mov %%ax, %%fs\n" \
				 "mov %%ax, %%gs\n" \
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

// Writes a 64-bit value to a control register using XSETBV.
static inline void asm_wrxcr(u32 reg, u64 val)
{
	u32 eax = val;
	u32 edx = val >> 32;
	asm volatile("xsetbv" ::"a"(eax), "d"(edx), "c"(reg) : "memory");
}

// Saves the FPU state to a 512-byte region of memory using FXSAVE.
// Pointer must be 16-byte aligned.
static inline void asm_fpu_fxsave(void* mem)
{
	asm volatile("fxsave %0" : "+m"(*(u8*)mem)::"memory");
}

// Restores the FPU state from a 512-byte region of memory using FXRSTOR.
// Pointer must be 16-byte aligned.
static inline void asm_fpu_fxrstor(void* mem)
{
	asm volatile("fxrstor %0" ::"m"(*(u8*)mem) : "memory");
}

// Saves the FPU state to a region of memory using XSAVE.
// Pointer must be 16-byte aligned.
static inline void asm_fpu_xsave(void* mem)
{
	asm volatile("xsave %0" : "+m"(*(u8*)mem)::"memory");
}

// Restores the FPU state from a region of memory using XRSTOR.
// Pointer must be 16-byte aligned.
static inline void asm_fpu_xrstor(void* mem)
{
	asm volatile("xrstor %0" ::"m"(*(u8*)mem) : "memory");
}

static inline u8 asm_read8(u16 port)
{
	u8 result;
	asm volatile("inb %1, %0" : "=a"(result) : "Nd"(port));
	return result;
}

static inline u16 asm_read16(u16 port)
{
	u16 result;
	asm volatile("inw %1, %0" : "=a"(result) : "Nd"(port));
	return result;
}

static inline u32 asm_read32(u16 port)
{
	u32 result;
	asm volatile("inl %1, %0" : "=a"(result) : "Nd"(port));
	return result;
}

static inline void asm_write8(u16 port, u8 value)
{
	asm volatile("outb %0, %1" : : "a"(value), "Nd"(port));
}

static inline void asm_write16(u16 port, u16 value)
{
	asm volatile("outw %0, %1" : : "a"(value), "Nd"(port));
}

static inline void asm_write32(u16 port, u32 value)
{
	asm volatile("outl %0, %1" : : "a"(value), "Nd"(port));
}
