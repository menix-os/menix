// x86 port IO.

#pragma once

#include <menix/common.h>

static inline u8 arch_x86_read8(u16 port)
{
	u8 result;
	asm volatile("inb %1, %0" : "=a"(result) : "Nd"(port));
	return result;
}

static inline u16 arch_x86_read16(u16 port)
{
	u16 result;
	asm volatile("inw %1, %0" : "=a"(result) : "Nd"(port));
	return result;
}

static inline u32 arch_x86_read32(u16 port)
{
	u32 result;
	asm volatile("inl %1, %0" : "=a"(result) : "Nd"(port));
	return result;
}

static inline void arch_x86_write8(u16 port, u8 value)
{
	asm volatile("outb %0, %1" : : "a"(value), "Nd"(port));
}

static inline void arch_x86_write16(u16 port, u16 value)
{
	asm volatile("outw %0, %1" : : "a"(value), "Nd"(port));
}

static inline void arch_x86_write32(u16 port, u32 value)
{
	asm volatile("outl %0, %1" : : "a"(value), "Nd"(port));
}
