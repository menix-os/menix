// Link-time information about the kernel binary itself.
// These should be defined in the respective linker scripts.

#pragma once

#include <menix/util/types.h>

extern const u8 __ld_kernel_start;
extern const u8 __ld_kernel_end;

#define SECTION_DECLARE_SYMBOLS(section) \
	extern const u8 __ld_sect_##section##_start; \
	extern const u8 __ld_sect_##section##_end;

#define SECTION_START(section) (&__ld_sect_##section##_start)
#define SECTION_END(section)   (&__ld_sect_##section##_end)
#define SECTION_SIZE(section)  (SECTION_END(section) - SECTION_START(section))
