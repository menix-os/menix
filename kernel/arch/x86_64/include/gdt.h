// Global Descriptor Table management

#pragma once

#include <menix/common.h>

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

// These entries are ordered exactly like this because the SYSRET instruction
// expects it.
typedef struct ATTR(packed)
{
	GdtDesc null;			// Unused
	GdtDesc kernel_code;	// Kernel CS
	GdtDesc kernel_data;	// Kernel DS
	GdtDesc user_code;		// 32-bit compatibility mode user CS
	GdtDesc user_data;		// User DS
	GdtDesc user_code64;	// 64-bit user CS
	GdtLongDesc tss;		// Task state segment
} Gdt;

// GDT register
typedef struct ATTR(packed)
{
	u16 limit;	  // Should be set to the size of the GDT - 1.
	Gdt* base;	  // Start of the GDT.
} GdtRegister;

// Install the Global Descriptor Table.
void gdt_init();

// Reload the Global Descriptor Table and flush segment registers.
void gdt_reload();

// Loads a new TSS into the GDT.
void gdt_load_tss(usize addr);
