//? Interrupt descriptor table setting

#include <menix/io.h>

#include <arch_bits.h>
#include <idt.h>
#include <interrupts.h>

ATTR(aligned(0x10)) ATTR(section(".data")) IdtDesc idt_table[IDT_MAX_SIZE] = { 0 };

void idt_set(IdtDesc* target, void* handler, uint8_t type_attr)
{
	const size_t ptr = (size_t)handler;

	target->base_0_15 = (uint16_t)ptr;
	target->base_16_31 = ptr >> 16;
	target->selector = 8;
	target->type = type_attr;
	target->reserved = 0;
#ifdef CONFIG_64_bit
	target->base_32_63 = ptr >> 32;
	target->reserved2 = 0;
#endif
}

void idt_reload()
{
	IdtRegister idtr = {
		.limit = sizeof(idt_table) - 1,	   // Limit is the last entry, not total size.
		.base = idt_table,
	};
	asm volatile("lidt %0" ::"m"(idtr));
}

void idt_init()
{
	interrupt_disable();

	for (uint8_t i = 0; i <= 0x20; i++)
		idt_set(&idt_table[i], int_error_handler, IDT_TYPE(0, IDT_GATE_INT));

	// Interrupt 0x80 is syscall.
	idt_set(&idt_table[0x80], int_syscall_handler, IDT_TYPE(3, IDT_GATE_INT));

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
}
