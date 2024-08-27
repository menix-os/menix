// Interrupt descriptor table setting

#include <menix/io/mmio.h>
#include <menix/log.h>

#include <bits/arch.h>
#include <bits/asm.h>
#include <gdt.h>
#include <idt.h>
#include <interrupts.h>
#include <io.h>
#include <pic.h>

ATTR(aligned(CONFIG_page_size)) static IdtDesc idt_table[IDT_MAX_SIZE];
ATTR(aligned(0x10)) static IdtRegister idtr;

void idt_set(u8 idx, void* handler, u8 type_attr)
{
	IdtDesc* const target = idt_table + idx;
	const usize ptr = (usize)handler;

	target->base_0_15 = ptr & 0xFFFF;
	target->base_16_31 = (ptr >> 16) & 0xFFFF;
	target->selector = offsetof(Gdt, kernel_code);
	target->type = type_attr;
	target->reserved = 0;
#if CONFIG_bits >= 64
	target->base_32_63 = (ptr >> 32) & 0xFFFFFFFF;
	target->reserved2 = 0;
#endif
}

void idt_reload()
{
	idtr.limit = sizeof(idt_table) - 1;	   // Limit is the last entry, not total size.
	idtr.base = idt_table;
	asm volatile("lidt %0" ::"m"(idtr));
}

INT_HANDLER(0, error_handler);
INT_HANDLER(1, error_handler);
INT_HANDLER(2, error_handler);
INT_HANDLER(3, error_breakpoint_handler);
INT_HANDLER(4, error_handler);
INT_HANDLER(5, error_handler);
INT_HANDLER(6, error_handler_invalid_opcode);
INT_HANDLER(7, error_handler);
INT_HANDLER_WITH_CODE(8, error_handler_with_code);
INT_HANDLER(9, error_handler);
INT_HANDLER_WITH_CODE(10, error_handler_with_code);
INT_HANDLER_WITH_CODE(11, error_handler_with_code);
INT_HANDLER_WITH_CODE(12, error_handler_with_code);
INT_HANDLER_WITH_CODE(13, error_handler_with_code);
INT_HANDLER_WITH_CODE(14, vm_page_fault_handler);
INT_HANDLER(15, error_handler);
INT_HANDLER(16, error_handler);
INT_HANDLER_WITH_CODE(17, error_handler_with_code);
INT_HANDLER(18, error_handler);
INT_HANDLER(19, error_handler);
INT_HANDLER(20, error_handler);
INT_HANDLER_WITH_CODE(21, error_handler_with_code);
INT_HANDLER(22, error_handler);
INT_HANDLER(23, error_handler);
INT_HANDLER(24, error_handler);
INT_HANDLER(25, error_handler);
INT_HANDLER(26, error_handler);
INT_HANDLER(27, error_handler);
INT_HANDLER(28, error_handler);
INT_HANDLER_WITH_CODE(29, error_handler_with_code);
INT_HANDLER_WITH_CODE(30, error_handler_with_code);
INT_HANDLER(31, error_handler);

void idt_init()
{
	asm_interrupt_disable();

	// Set exception vector (0x00 - 0x1F)
	idt_set(0x00, int_0, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x01, int_1, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x02, int_2, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x03, int_3, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x04, int_4, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x05, int_5, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x06, int_6, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x07, int_7, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x08, int_8, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x09, int_9, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x0A, int_10, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x0B, int_11, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x0C, int_12, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x0D, int_13, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x0E, int_14, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x0F, int_15, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x10, int_16, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x11, int_17, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x12, int_18, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x13, int_19, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x14, int_20, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x15, int_21, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x16, int_22, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x17, int_23, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x18, int_24, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x19, int_25, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x1A, int_26, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x1B, int_27, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x1C, int_28, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x1D, int_29, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x1E, int_30, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x1F, int_31, IDT_TYPE(0, IDT_GATE_INT));

	// Interrupt 0x80 is syscall (Only for legacy invocations using "int $0x80").
	idt_set(0x80, int_syscall, IDT_TYPE(0, IDT_GATE_INT));

	arch_x86_write8(PIC1_COMMAND_PORT, 0x11);
	arch_x86_write8(PIC2_COMMAND_PORT, 0x11);
	arch_x86_write8(PIC1_DATA_PORT, 0x20);
	arch_x86_write8(PIC2_DATA_PORT, 0x28);
	arch_x86_write8(PIC1_DATA_PORT, 0x0);
	arch_x86_write8(PIC2_DATA_PORT, 0x0);
	arch_x86_write8(PIC1_DATA_PORT, 0x1);
	arch_x86_write8(PIC1_DATA_PORT, 0x1);
	arch_x86_write8(PIC1_DATA_PORT, 0xFF);
	arch_x86_write8(PIC1_DATA_PORT, 0xFF);

	idt_reload();
	asm_interrupt_enable();
}
