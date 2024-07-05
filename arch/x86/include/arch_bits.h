//? x86 specific bits of code

#pragma once

#include <menix/common.h>

uint8_t	 arch_read8(uint16_t port);
uint16_t arch_read16(uint16_t port);
uint32_t arch_read32(uint16_t port);
#ifdef CONFIG_64_bit
uint32_t arch_read64(uint16_t port);
#endif

void arch_write8(uint16_t port, uint8_t value);
void arch_write16(uint16_t port, uint16_t value);
void arch_write32(uint16_t port, uint32_t value);
#ifdef CONFIG_64_bit
void arch_write64(uint16_t port, uint32_t value);
#endif

#define interrupt_disable() asm volatile("cli")
#define interrupt_enable()	asm volatile("sti")
