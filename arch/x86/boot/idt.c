//? Interrupt descriptor table setting

#include <menix/syscalls.h>

#include <arch_bits.h>
#include <idt.h>
#include <interrupts.h>

ATTR(aligned(0x10)) static IdtDesc	   idt_table[IDT_MAX_SIZE] = { 0 };
ATTR(aligned(0x10)) static IdtRegister idtr = { .limit = sizeof(idt_table), .base = idt_table };

void idt_fill(IdtDesc* target, void* offset, uint16_t selector, uint8_t type_attr)
{
	target->offset_0_15 = (size_t)offset & 0xFFFF;
	target->offset_16_31 = (size_t)offset >> 16;
	target->selector = selector;
	target->type_attr = type_attr;
	target->zero = 0;
}

void idt_init()
{
	// Install IDT.
	// TODO: For now, fill all handlers with the error handler.
	for (uint8_t i = 0; i < 0x20; i++)
		idt_fill(&idt_table[i], error_handler, KERNEL_CODE_SEGMENT_OFFSET, IDT_INTERRUPT_GATE_32);

	// Interrupt 0x80 is syscall.
	idt_fill(&idt_table[0x80], syscall_handler, KERNEL_CODE_SEGMENT_OFFSET, IDT_INTERRUPT_GATE_32);

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

	interrupt_disable();
	idt_set();
	interrupt_enable();
}

void idt_set()
{
	asm("lidt %0" ::"m"(idtr));
}
