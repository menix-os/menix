// Global Descriptor table

#include <menix/common.h>
#include <menix/system/arch.h>
#include <menix/thread/spin.h>

#include <gdt.h>
#include <tss.h>

ATTR(aligned(0x1000)) Gdt gdt_table;
ATTR(aligned(0x10)) TaskStateSegment tss;
ATTR(aligned(0x10)) GdtRegister gdtr = {
	.limit = sizeof(gdt_table) - 1,
	.base = &gdt_table,
};

void gdt_init()
{
	// clang-format off

	asm_interrupt_disable();
	// Kernel Code
	GDT_ENCODE(gdt_table.kernel_code,
		0, 0xFFFFF,
		GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
		GDTF_GRANULARITY | GDTF_LONG_MODE);

	// Kernel Data
	GDT_ENCODE(gdt_table.kernel_data,
		0, 0xFFFFF,
		GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_SEGMENT | GDTA_READ_WRITE,
		GDTF_GRANULARITY | GDTF_LONG_MODE);

	// User Code 32-bit
	GDT_ENCODE(gdt_table.user_code,
		0, 0xFFFFF,
		GDTA_PRESENT | GDTA_PRIV_LVL(3) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
		GDTF_GRANULARITY | GDTF_PROT_MODE);

	// User Data
	GDT_ENCODE(gdt_table.user_data,
		0, 0xFFFFF,
		GDTA_PRESENT | GDTA_PRIV_LVL(3) | GDTA_SEGMENT | GDTA_READ_WRITE,
		GDTF_GRANULARITY | GDTF_LONG_MODE);

	// User Code 64-bit
	GDT_ENCODE(gdt_table.user_code64,
		0, 0xFFFFF,
		GDTA_PRESENT | GDTA_PRIV_LVL(3) | GDTA_SEGMENT | GDTA_EXECUTABLE | GDTA_READ_WRITE,
		GDTF_GRANULARITY | GDTF_LONG_MODE);

	// Task State Segment (TSS)
	tss_init(&tss);
	GDT_ENCODE_LONG(gdt_table.tss,
		(u64)&tss, sizeof(TaskStateSegment),
		GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_EXECUTABLE | GDTA_ACCESSED,
		0);

	// clang-format on

	gdt_reload();
	tss_reload();
	asm_interrupt_enable();
}

void gdt_reload()
{
	asm_gdt_set(gdtr);
	asm_flush_segment_regs(offsetof(Gdt, kernel_code), offsetof(Gdt, kernel_data));
}

static SpinLock gdt_lock = spin_new();

void gdt_load_tss(usize addr)
{
	spin_acquire_force(&gdt_lock);
	GDT_ENCODE_LONG(gdt_table.tss, addr, sizeof(TaskStateSegment),
					GDTA_PRESENT | GDTA_PRIV_LVL(0) | GDTA_EXECUTABLE | GDTA_ACCESSED, 0);
	tss_reload();
	spin_free(&gdt_lock);
}
