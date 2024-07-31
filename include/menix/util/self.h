// Link/Compile-time information about the kernel binary itself.
// These should be defined in the respective linker scripts.

#pragma once

#include <menix/thread/elf.h>
#include <menix/util/types.h>

extern volatile const u8 __ld_kernel_start;
extern volatile const u8 __ld_kernel_end;

#define SECTION_DECLARE_SYMBOLS(section) \
	extern volatile const u8 __ld_sect_##section##_start; \
	extern volatile const u8 __ld_sect_##section##_end;

#define SECTION_START(section) (&__ld_sect_##section##_start)
#define SECTION_END(section)   (&__ld_sect_##section##_end)
#define SECTION_SIZE(section)  (SECTION_END(section) - SECTION_START(section))

// Sets the current kernel context to the given address.
void self_set_kernel(Elf_Hdr* addr);

// Returns a pointer to where the kernel was loaded into memory.
Elf_Hdr* self_get_kernel();
