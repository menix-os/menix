// x86 port IO.

#pragma once

#include <menix/common.h>

ATTR(always_inline) u8 arch_x86_read8(u16 port);
ATTR(always_inline) u16 arch_x86_read16(u16 port);
ATTR(always_inline) u32 arch_x86_read32(u16 port);

ATTR(always_inline) void arch_x86_write8(u16 port, u8 value);
ATTR(always_inline) void arch_x86_write16(u16 port, u16 value);
ATTR(always_inline) void arch_x86_write32(u16 port, u32 value);
