//? x86 specific bits of code

#pragma once

#include <menix/common.h>

// Reads 8 bits from a given IO port.
uint8_t	 read8(uint16_t port);
uint16_t read16(uint16_t port);
uint32_t read32(uint16_t port);

// Writes 8 bits to a given IO port.
void write8(uint16_t port, uint8_t value);
void write16(uint16_t port, uint16_t value);
void write32(uint16_t port, uint32_t value);

#define interrupt_disable() asm volatile("cli")
#define interrupt_enable()	asm volatile("sti")
