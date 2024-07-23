// x86 port IO.

#pragma once

#include <menix/common.h>

uint8_t arch_read8(uint16_t port);
uint16_t arch_read16(uint16_t port);
uint32_t arch_read32(uint16_t port);

void arch_write8(uint16_t port, uint8_t value);
void arch_write16(uint16_t port, uint16_t value);
void arch_write32(uint16_t port, uint32_t value);
