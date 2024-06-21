//? Interrupt descriptor table setting

#include <menix/syscalls.h>

#include <arch_bits.h>
#include <idt.h>

void idt_fill(uint8_t idx, uint32_t offset, uint16_t selector, uint8_t type_attr)
{
	idt_table[idx].offset_low = offset & 0xFFFF;
	idt_table[idx].offset_high = offset >> 16;
	idt_table[idx].selector = selector;
	idt_table[idx].type_attr = type_attr;
	idt_table[idx].zero = 0;
}

void idt_init()
{
	// Install IDT.
	// TODO: For now, fill all handlers with the error handler.
	for (uint8_t i = 0; i < 0x20; i++)
		idt_fill(i, (uint32_t)error_handler, KERNEL_CODE_SEGMENT_OFFSET, IDT_INTERRUPT_GATE_32);

	// Interrupt 0x80 is syscall.
	idt_fill(0x80, (uint32_t)syscall_handler, KERNEL_CODE_SEGMENT_OFFSET, IDT_INTERRUPT_GATE_32);

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
	idt_set((sizeof(IdtEntry) * IDT_MAX_SIZE) - 1, (uint32_t)idt_table);
	interrupt_enable();
}
