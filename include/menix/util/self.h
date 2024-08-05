// Link/Compile-time information about the kernel binary itself.
// These should be defined in the respective linker scripts.

#pragma once

#include <menix/thread/elf.h>
#include <menix/util/types.h>

extern const u8 __ld_kernel_start[];
extern const u8 __ld_kernel_end[];

#define KERNEL_START (__ld_kernel_start)
#define KERENL_END	 (__ld_kernel_end)

#define SECTION_DECLARE_SYMBOLS(section) \
	extern const u8 __ld_sect_##section##_start[]; \
	extern const u8 __ld_sect_##section##_end[];

#define SECTION_START(section) (__ld_sect_##section##_start)
#define SECTION_END(section)   (__ld_sect_##section##_end)
#define SECTION_SIZE(section)  (SECTION_END(section) - SECTION_START(section))

#define SEGMENT_DECLARE_SYMBOLS(segment) \
	extern const u8 __ld_seg_##segment##_start[]; \
	extern const u8 __ld_seg_##segment##_end[];

#define SEGMENT_START(segment) (__ld_seg_##segment##_start)
#define SEGMENT_END(segment)   (__ld_seg_##segment##_end)
#define SEGMENT_SIZE(segment)  (SECTION_END(segment) - SECTION_START(segment))

// Sets the current kernel context to the given address.
void self_set_kernel(Elf_Hdr* addr);

// Returns a pointer to where the kernel was loaded into memory.
Elf_Hdr* self_get_kernel();
