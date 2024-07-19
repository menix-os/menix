//? Global Descriptor table

#include <menix/common.h>

#include <bits/arch.h>
#include <gdt.h>
#include <string.h>
#include <tss.h>

ATTR(aligned(0x10)) GdtDesc		gdt_table[6];
ATTR(aligned(0x10)) GdtRegister gdtr = {
	.limit = sizeof(gdt_table) - 1,
	.base = gdt_table,
};
ATTR(aligned(0x10)) TaskStateSegment tss;

void gdt_fill(uint8_t idx, void* base, uint32_t limit, uint8_t access, uint8_t flags)
{
	GdtDesc* const target = gdt_table + idx;
	const size_t   ptr = (size_t)base;

	target->limit_0_15 = limit & 0xFFFF;
	target->base_0_23 = ptr & 0xFFFFFF;
	target->access = access;
	target->limit_16_19 = (limit >> 16) & 0xFFFF;
	target->flags = flags;
	target->base_24_31 = (ptr >> 24) & 0xFFFFFF;
}

void gdt_init()
{
	interrupt_disable();
	// Kernel Code
	gdt_fill(GDT_KERNEL_CODE, NULL, 0xFFFFF,
			 GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
			 GDTF_GRANULARITY | GDTF_LONG_MODE);

	// Kernel Data
	gdt_fill(GDT_KERNEL_DATA, NULL, 0xFFFFF, GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_SEGMENT | GDTA_READ_WRITE,
			 GDTF_GRANULARITY | GDTF_PROT_MODE);

	// User Code
	gdt_fill(GDT_USER_CODE, NULL, 0xFFFFF,
			 GDTA_PRESENT | GDTA_PRIV_LVL(3) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
			 GDTF_GRANULARITY | GDTF_LONG_MODE);

	// User Data
	gdt_fill(GDT_USER_DATA, NULL, 0xFFFFF, GDTA_PRESENT | GDTA_PRIV_LVL(3) | GDTA_SEGMENT | GDTA_READ_WRITE,
			 GDTF_GRANULARITY | GDTF_PROT_MODE);

	// Task State Segment (TSS)
	memset(&tss, 0, sizeof(TaskStateSegment));
	gdt_fill(GDT_TSS, &tss, sizeof(TaskStateSegment), GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_EXECUTABLE | GDTA_ACCESSED,
			 0);

	gdt_set(gdtr);
	gdt_flush_regs(GDT_OFFSET(GDT_KERNEL_CODE), GDT_OFFSET(GDT_KERNEL_DATA));
	interrupt_enable();
}
