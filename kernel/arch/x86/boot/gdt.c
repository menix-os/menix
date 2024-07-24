// Global Descriptor table

#include <menix/common.h>

#include <bits/asm.h>
#include <gdt.h>
#include <tss.h>

ATTR(aligned(CONFIG_page_size)) Gdt gdt_table;
ATTR(aligned(0x10)) TaskStateSegment tss;
ATTR(aligned(0x10)) GdtRegister gdtr = {
	.limit = sizeof(gdt_table) - 1,
	.base = &gdt_table,
};

void gdt_init()
{
	interrupt_disable();
	// Kernel Code
	GDT_ENCODE(gdt_table.kernel_code, 1, 0xFFFFF,
			   GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
			   GDTF_GRANULARITY | GDTF_LONG_MODE);

	// Kernel Data
	GDT_ENCODE(gdt_table.kernel_data, 0, 0xFFFFF, GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_SEGMENT | GDTA_READ_WRITE,
			   GDTF_GRANULARITY | GDTF_PROT_MODE);

	// User Code
	GDT_ENCODE(gdt_table.user_code, 0, 0xFFFFF,
			   GDTA_PRESENT | GDTA_PRIV_LVL(3) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
			   GDTF_GRANULARITY | GDTF_LONG_MODE);

	// User Data
	GDT_ENCODE(gdt_table.user_data, 0, 0xFFFFF, GDTA_PRESENT | GDTA_PRIV_LVL(3) | GDTA_SEGMENT | GDTA_READ_WRITE,
			   GDTF_GRANULARITY | GDTF_PROT_MODE);

	// Task State Segment (TSS)
	tss_init(&tss);
	GDT_ENCODE_LONG(gdt_table.tss, (u64)&tss, sizeof(TaskStateSegment),
					GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_EXECUTABLE | GDTA_ACCESSED, 0);

	gdt_set(gdtr);
	flush_segment_regs(offsetof(Gdt, kernel_code), offsetof(Gdt, kernel_data));
	interrupt_enable();
}
