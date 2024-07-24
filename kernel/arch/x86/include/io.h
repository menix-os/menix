// x86 port IO.

#pragma once

#include <menix/common.h>

u8 arch_read8(u16 port);
u16 arch_read16(u16 port);
u32 arch_read32(u16 port);

void arch_write8(u16 port, u8 value);
void arch_write16(u16 port, u16 value);
void arch_write32(u16 port, u32 value);
