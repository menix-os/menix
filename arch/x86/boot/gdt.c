//? Global Descriptor table

#include <menix/common.h>

#include <gdt.h>
#include <tss.h>

ATTR(aligned(0x10)) static GdtDesc			gdt_table[7] = { 0 };
ATTR(aligned(0x10)) static GdtRegister		gdtr = { .limit = sizeof(gdt_table), .base = gdt_table };
ATTR(aligned(0x10)) static TaskStateSegment tss = { 0 };

void gdt_fill(GdtDesc* target, void* base, uint32_t limit, uint8_t access, uint8_t flags)
{
	target->limit_0_15 = limit & 0xFFFF;
	target->base_0_23 = (size_t)base & 0xFFFFFF;
	target->access = access;
	target->limit_16_19 = (limit & 0xF0000) >> 16;
	target->flags = flags;
	target->base_24_31 = ((size_t)base & 0xF000000) >> 24;
#ifdef CONFIG_64_bit
	target->base_32_63 = ((size_t)base & 0xFFFFFFFF00000000) >> 32;
#endif
}

void gdt_init()
{
	// clang-format off

	// Kernel Code
	gdt_fill(&gdt_table[GDT_KERNEL_CODE],
			 NULL, 0xFFFFF,
			 GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
			 GDTF_GRANULARITY |
#ifdef CONFIG_64_bit
			 GDTF_LONG_MODE
#else
			 GDTF_SIZE
#endif
	);

	// Kernel Data
	gdt_fill(&gdt_table[GDT_KERNEL_DATA],
			 NULL, 0xFFFFF,
			 GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
			 GDTF_GRANULARITY | GDTF_SIZE);

	// Kernel Stack
	gdt_fill(&gdt_table[GDT_KERNEL_STACK],
			 NULL, 0xFFFFF,
			 GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
			 GDTF_GRANULARITY | GDTF_SIZE);

	// User Code
	gdt_fill(&gdt_table[GDT_USER_CODE],
			 NULL, 0xFFFFF,
			 GDTA_PRESENT | GDTA_PRIV_LVL(3) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
			 GDTF_GRANULARITY |
#ifdef CONFIG_64_bit
			 GDTF_LONG_MODE
#else
			 GDTF_SIZE
#endif
	);

	// User Data
	gdt_fill(&gdt_table[GDT_USER_DATA],
			 NULL, 0xFFFFF,
			 GDTA_PRESENT | GDTA_PRIV_LVL(3) | GDTA_SEGMENT | GDTA_READ_WRITE,
			 GDTF_GRANULARITY | GDTF_SIZE);

	// Task State Segment (TSS)
	gdt_fill(&gdt_table[GDT_TSS],
			 &tss, sizeof(TaskStateSegment),
			 GDTA_PRESENT | GDTA_EXECUTABLE | GDTA_ACCESSED,
			 0);

	// clang-format on

	gdt_set(gdtr);
}
