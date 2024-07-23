// Port IO for x86

#include <menix/common.h>

#include <bits/arch.h>

uint8_t arch_read8(uint16_t port)
{
	uint8_t result;
	asm volatile("inb %1, %0" : "=a"(result) : "Nd"(port));
	return result;
}

uint16_t arch_read16(uint16_t port)
{
	uint16_t result;
	asm volatile("inw %1, %0" : "=a"(result) : "Nd"(port));
	return result;
}

uint32_t arch_read32(uint16_t port)
{
	uint32_t result;
	asm volatile("inl %1, %0" : "=a"(result) : "Nd"(port));
	return result;
}

void arch_write8(uint16_t port, uint8_t value)
{
	asm volatile("outb %0, %1" : : "a"(value), "Nd"(port));
}

void arch_write16(uint16_t port, uint16_t value)
{
	asm volatile("outw %0, %1" : : "a"(value), "Nd"(port));
}

void arch_write32(uint16_t port, uint32_t value)
{
	asm volatile("outl %0, %1" : : "a"(value), "Nd"(port));
}
