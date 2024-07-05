//? Port IO for x86

#include <menix/common.h>

#include <arch_bits.h>

uint8_t arch_read8(uint16_t port)
{
	uint8_t result;
	asm volatile("movw %0, %%dx" ::"r"(port));		// Move "port" into dx (mandatory).
	asm volatile("inb %%dx, %0" : "=r"(result));	// Get byte from "port" and store in "result".
	return result;
}

uint16_t arch_read16(uint16_t port)
{
	uint16_t result;
	asm volatile("movw %0, %%dx" ::"r"(port));
	asm volatile("inw %%dx, %0" : "=r"(result));
	return result;
}

uint32_t arch_read32(uint16_t port)
{
	uint32_t result;
	asm volatile("movw %0, %%dx" ::"r"(port));
	asm volatile("inl %%dx, %0" : "=r"(result));
	return result;
}

void arch_write8(uint16_t port, uint8_t value)
{
	asm volatile("movw %0, %%dx" ::"r"(port));
	asm volatile("outb %0, %%dx" ::"r"(value));
}

void arch_write16(uint16_t port, uint16_t value)
{
	asm volatile("movw %0, %%dx" ::"r"(port));
	asm volatile("outw %0, %%dx" ::"r"(value));
}

void arch_write32(uint16_t port, uint32_t value)
{
	asm volatile("movw %0, %%dx" ::"r"(port));
	asm volatile("outl %0, %%dx" ::"r"(value));
}

#ifdef CONFIG_64_bit
void arch_write64(uint16_t port, uint32_t value)
{
	asm volatile("movw %0, %%dx" ::"r"(port));
	asm volatile("outl %0, %%dx" ::"r"(value));
}
#endif
