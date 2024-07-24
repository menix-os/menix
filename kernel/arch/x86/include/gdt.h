// Global Descriptor Table management

#pragma once

#include <menix/common.h>

// Indices of the GDT segments
#define GDT_KERNEL_CODE 1
#define GDT_KERNEL_DATA 2
#define GDT_USER_CODE	3
#define GDT_USER_DATA	4
#define GDT_TSS			5

// Gets the offset in bytes relative to the GDTR.
#define GDT_OFFSET(entry) (entry * sizeof(GdtDesc))

#define GDTA_PRESENT	   (1 << 7)
#define GDTA_PRIV_LVL(lvl) ((lvl & 3) << 5)
#define GDTA_SEGMENT	   (1 << 4)
#define GDTA_EXECUTABLE	   (1 << 3)
#define GDTA_DIR_CONF	   (1 << 2)
#define GDTA_READ_WRITE	   (1 << 1)
#define GDTA_ACCESSED	   (1 << 0)

#define GDTF_GRANULARITY (1 << 3)
// 0 = 16-bit, 1 = 32-bit protected-mode segment
#define GDTF_PROT_MODE	 (1 << 2)
#define GDTF_LONG_MODE	 (1 << 1)

#define GDT_ENCODE(target, base, limit, access_byte, flags_attr) \
	target = (GdtDesc) \
	{ \
		.limit0 = (limit) & 0xFFFF, .base0 = (base) & 0xFFFFFF, .access = (access_byte), \
		.limit1 = (limit >> 16) & 0xF, .flags = (flags_attr), .base1 = (base >> 24) & 0xFFFFFF, \
	}

#define GDT_ENCODE_LONG(target, base, limit, access_byte, flags_attr) \
	target = (GdtLongDesc) \
	{ \
		.limit0 = (limit) & 0xFFFF, .base0 = (base) & 0xFFFFFF, .access = (access_byte), \
		.limit1 = (limit >> 16) & 0xF, .flags = (flags_attr), .base1 = (base >> 24) & 0xFFFFFF, \
		.base2 = (base >> 32) & 0xFFFFFFFF, \
	}

// GDT segment descriptor
typedef struct ATTR(packed)
{
	Bits limit0:16;	   // Limit[0..15]
	Bits base0:24;	   // Base[0..23]
	Bits access:8;	   // Access modifider
	Bits limit1:4;	   // Limit[16..19]
	Bits flags:4;	   // Flags
	Bits base1:8;	   // Base[24..31]
} GdtDesc;

// Long mode GDT segment descriptor
typedef struct ATTR(packed)
{
	Bits limit0:16;	   // Limit[0..15]
	Bits base0:24;	   // Base[0..23]
	Bits access:8;	   // Access modifider
	Bits limit1:4;	   // Limit[16..19]
	Bits flags:4;	   // Flags
	Bits base1:8;	   // Base[24..31]
	Bits base2:32;	   // Base[32..63]
	Bits reserved;	   // Reserved
} GdtLongDesc;

typedef struct ATTR(packed)
{
	GdtDesc null;
	GdtDesc kernel_code;
	GdtDesc kernel_data;
	GdtDesc user_code;
	GdtDesc user_data;
	GdtLongDesc tss;
} Gdt;

// GDT register
typedef struct ATTR(packed)
{
	u16 limit;	  // Should be set to the size of the GDT - 1.
	Gdt* base;	  // Start of the GDT.
} GdtRegister;

// Fills the GDT with predefined values.
void gdt_init();
