//? Interrupt descriptor table setting

#include <menix/io.h>
#include <menix/log.h>

#include <bits/arch.h>
#include <idt.h>
#include <interrupts.h>

#include "gdt.h"

ATTR(aligned(0x10)) static IdtDesc	   idt_table[IDT_MAX_SIZE];
ATTR(aligned(0x10)) static IdtRegister idtr;

void idt_set(uint8_t idx, void* handler, uint8_t type_attr)
{
	kassert(handler != NULL, "IDT Entry: Function pointer is null???\n");

	IdtDesc* const target = idt_table + idx;
	const size_t   ptr = (size_t)handler;

	target->base_0_15 = ptr & 0xFFFF;
	target->base_16_31 = (ptr >> 16) & 0xFFFF;
	target->selector = GDT_OFFSET(GDT_KERNEL_CODE);
	target->type = type_attr;
	target->reserved = 0;
#ifdef CONFIG_64_bit
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

//! Here are a few inline assembly macros to create some stubs since we have so many of them.

#define INT_HANDLER_DECL(num) extern void int_error_handler_##num(void);
#define INT_HANDLER_COMMON(num) \
	".global int_error_handler_" #num "\n" \
	".align 0x10\n" \
	"int_error_handler_" #num ":\n"

#define INT_HANDLER(num) \
	INT_HANDLER_DECL(num) \
	asm(INT_HANDLER_COMMON(num) "mov $" #num ", %rdi\n" \
								"call error_handler\n" \
								"iretq\n")

#define INT_HANDLER_WITH_CODE(num) \
	INT_HANDLER_DECL(num) \
	asm(INT_HANDLER_COMMON(num) "mov $" #num ", %rdi\n" \
								"pop %rsi\n" \
								"call error_handler_with_code\n" \
								"iretq\n")

INT_HANDLER(0);
INT_HANDLER(1);
INT_HANDLER(2);
INT_HANDLER(3);
INT_HANDLER(4);
INT_HANDLER(5);
INT_HANDLER(6);
INT_HANDLER(7);
INT_HANDLER_WITH_CODE(8);
INT_HANDLER(9);
INT_HANDLER_WITH_CODE(10);
INT_HANDLER_WITH_CODE(11);
INT_HANDLER_WITH_CODE(12);
INT_HANDLER_WITH_CODE(13);
INT_HANDLER_WITH_CODE(14);
INT_HANDLER(15);
INT_HANDLER(16);
INT_HANDLER_WITH_CODE(17);
INT_HANDLER(18);
INT_HANDLER(19);
INT_HANDLER(20);
INT_HANDLER_WITH_CODE(21);
INT_HANDLER(22);
INT_HANDLER(23);
INT_HANDLER(24);
INT_HANDLER(25);
INT_HANDLER(26);
INT_HANDLER(27);
INT_HANDLER(28);
INT_HANDLER_WITH_CODE(29);
INT_HANDLER_WITH_CODE(30);
INT_HANDLER(31);

void idt_init()
{
	interrupt_disable();

	// Set exception vector (0x00 - 0x1F)
	idt_set(0x00, int_error_handler_0, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x01, int_error_handler_1, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x02, int_error_handler_2, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x03, int_error_handler_3, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x04, int_error_handler_4, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x05, int_error_handler_5, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x06, int_error_handler_6, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x07, int_error_handler_7, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x08, int_error_handler_8, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x09, int_error_handler_9, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x0A, int_error_handler_10, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x0B, int_error_handler_11, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x0C, int_error_handler_12, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x0D, int_error_handler_13, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x0E, int_error_handler_14, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x0F, int_error_handler_15, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x10, int_error_handler_16, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x11, int_error_handler_17, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x12, int_error_handler_18, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x13, int_error_handler_19, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x14, int_error_handler_20, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x15, int_error_handler_21, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x16, int_error_handler_22, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x17, int_error_handler_23, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x18, int_error_handler_24, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x19, int_error_handler_25, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x1A, int_error_handler_26, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x1B, int_error_handler_27, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x1C, int_error_handler_28, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x1D, int_error_handler_29, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x1E, int_error_handler_30, IDT_TYPE(0, IDT_GATE_INT));
	idt_set(0x1F, int_error_handler_31, IDT_TYPE(0, IDT_GATE_INT));

	// Interrupt 0x80 is syscall.
	idt_set(0x80, int_syscall_handler, IDT_TYPE(0, IDT_GATE_INT));

	write8(PIC1_COMMAND_PORT, 0x11);
	write8(PIC2_COMMAND_PORT, 0x11);
	write8(PIC1_DATA_PORT, 0x20);
	write8(PIC2_DATA_PORT, 0x28);
	write8(PIC1_DATA_PORT, 0x0);
	write8(PIC2_DATA_PORT, 0x0);
	write8(PIC1_DATA_PORT, 0x1);
	write8(PIC1_DATA_PORT, 0x1);
	write8(PIC1_DATA_PORT, 0xFF);
	write8(PIC1_DATA_PORT, 0xFF);

	idt_reload();
	interrupt_enable();
}
