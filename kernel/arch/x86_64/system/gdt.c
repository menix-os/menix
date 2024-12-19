// Global Descriptor table

#include <menix/common.h>
#include <menix/system/arch.h>
#include <menix/util/spin.h>

#include <gdt.h>
#include <tss.h>

void gdt_init(Gdt* gdt_table, TaskStateSegment* tss)
{
	// clang-format off
	asm_interrupt_disable();

	// Kernel Code
	GDT_ENCODE(gdt_table->kernel_code,
		0, 0xFFFFF,
		GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
		GDTF_GRANULARITY | GDTF_LONG_MODE);

	// Kernel Data
	GDT_ENCODE(gdt_table->kernel_data,
		0, 0xFFFFF,
		GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_SEGMENT | GDTA_READ_WRITE,
		GDTF_GRANULARITY | GDTF_LONG_MODE);

	// User Code 32-bit
	GDT_ENCODE(gdt_table->user_code,
		0, 0xFFFFF,
		GDTA_PRESENT | GDTA_PRIV_LVL(3) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
		GDTF_GRANULARITY | GDTF_PROT_MODE);

	// User Data
	GDT_ENCODE(gdt_table->user_data,
		0, 0xFFFFF,
		GDTA_PRESENT | GDTA_PRIV_LVL(3) | GDTA_SEGMENT | GDTA_READ_WRITE,
		GDTF_GRANULARITY | GDTF_LONG_MODE);

	// User Code 64-bit
	GDT_ENCODE(gdt_table->user_code64,
		0, 0xFFFFF,
		GDTA_PRESENT | GDTA_PRIV_LVL(3) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
		GDTF_GRANULARITY | GDTF_LONG_MODE);

	// Task State Segment (TSS)
	tss_init(tss);
	GDT_ENCODE_LONG(gdt_table->tss,
		(u64)tss, sizeof(TaskStateSegment),
		GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_EXECUTABLE | GDTA_ACCESSED,
		0);

	// clang-format on
	asm_interrupt_enable();
}

void gdt_load(Gdt* gdt_table)
{
	asm_interrupt_disable();
	GdtRegister gdtr = {
		.limit = sizeof(Gdt) - 1,
		.base = gdt_table,
	};

	asm_gdt_set(gdtr);
	tss_reload();
	asm_flush_segment_regs(offsetof(Gdt, kernel_code), offsetof(Gdt, kernel_data));
	asm_interrupt_enable();
}
