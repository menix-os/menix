//? Port IO for x86

#include <menix/common.h>

#include <arch_bits.h>

uint8_t read8(uint16_t port)
{
	uint8_t result;
	asm volatile("inb %1, %0" : "=a"(result) : "Nd"(port));
	return result;
}

uint16_t read16(uint16_t port)
{
	uint16_t result;
	asm volatile("inw %1, %0" : "=a"(result) : "Nd"(port));
	return result;
}

uint32_t read32(uint16_t port)
{
	uint32_t result;
	asm volatile("inl %1, %0" : "=a"(result) : "Nd"(port));
	return result;
}

#ifdef CONFIG_64_bit
uint64_t read64(uint16_t port)
{
	uint64_t result;
	result = (uint64_t)read32(port) << 32;
	result |= read32(port);
	return result;
}
#endif

void write8(uint16_t port, uint8_t value)
{
	asm volatile("outb %0, %1" : : "a"(value), "Nd"(port));
}

void write16(uint16_t port, uint16_t value)
{
	asm volatile("outw %0, %1" : : "a"(value), "Nd"(port));
}

void write32(uint16_t port, uint32_t value)
{
	asm volatile("outl %0, %1" : : "a"(value), "Nd"(port));
}

#ifdef CONFIG_64_bit
void write64(uint16_t port, uint64_t value)
{
	write32(port, value >> 32);
	write32(port, value & 0xFFFFFFFF);
}
#endif
