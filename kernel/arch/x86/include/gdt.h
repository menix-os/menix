/*--------------------------------
Global Descriptor Table management
--------------------------------*/

#pragma once

#include <menix/stdint.h>

// Encodes a GDT entry to be in the correct format.
#define GDT_ENTRY(base, limit, access, flags) \
		limit & 0xFF, (limit >> 8) & 0xFF, base & 0xFF, (base >> 8) & 0xFF, \
		(base >> 16) & 0xFF, access, flags << 4, (base >> 24) & 0xFF

static uint8_t gdt_table[] =
{
	GDT_ENTRY(0, 0, 0, 0),				// Null
	GDT_ENTRY(0, 0xFFFFF, 0x9A, 0xC),	// Kernel Code
	GDT_ENTRY(0, 0xFFFFF, 0x92, 0xC),	// Kernel Data
	GDT_ENTRY(0, 0xFFFFF, 0xFA, 0xC),	// User Code
	GDT_ENTRY(0, 0xFFFFF, 0xF2, 0xC)	// User Data
};

/// \brief	Sets the GDT on the CPU.
void gdt_set(uint32_t limit, uint32_t base);
