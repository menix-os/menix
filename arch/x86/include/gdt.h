//? Global Descriptor Table management

#pragma once

#include <menix/common.h>

// Indices of the GDT segments
#define GDT_KERNEL_CODE	 1
#define GDT_KERNEL_DATA	 2
#define GDT_KERNEL_STACK 3
#define GDT_USER_CODE	 4
#define GDT_USER_DATA	 5
#define GDT_TSS			 6

#define GDTA_PRESENT	   (1 << 7)
#define GDTA_PRIV_LVL(lvl) ((lvl & 3) << 5)
#define GDTA_SEGMENT	   (1 << 4)
#define GDTA_EXECUTABLE	   (1 << 3)
#define GDTA_DIR_CONF	   (1 << 2)
#define GDTA_READ_WRITE	   (1 << 1)
#define GDTA_ACCESSED	   (1 << 0)

#define GDTF_GRANULARITY (1 << 3)
// 0 = 16-bit, 1 = 32-bit protected-mode segment
#define GDTF_SIZE		 (1 << 2)
#define GDTF_LONG_MODE	 (1 << 1)

/// \brief GDT Segment Descriptor
typedef struct ATTR(packed)
{
	bits limit_0_15:16;
	bits base_0_23:24;
	bits access:8;
	bits limit_16_19:4;
	bits flags:4;
	bits base_24_31:8;
#ifdef CONFIG_64_bit
	bits base_32_63:32;
	bits reserved:32;
#endif
} GdtDesc;

/// \brief GDT Register emulation so it can be accessed from C.
typedef struct ATTR(packed)
{
	uint16_t limit;
	GdtDesc* base;
} GdtRegister;

/// \brief Encodes a GDT entry to be in the correct format.
void gdt_fill(GdtDesc* target, void* base, uint32_t limit, uint8_t access, uint8_t flags);

/// \brief Sets the GDT on the CPU.
void gdt_set();

/// \brief Fills the GDT with predefined values.
void gdt_init();
