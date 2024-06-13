/*--------------------------------
Interrupt descriptor table setting
--------------------------------*/

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

	io_out(PIC1_COMMAND_PORT, 0x11);
	io_out(PIC2_COMMAND_PORT, 0x11);
	io_out(PIC1_DATA_PORT, 0x20);
	io_out(PIC2_DATA_PORT, 0x28);
	io_out(PIC1_DATA_PORT, 0x0);
	io_out(PIC2_DATA_PORT, 0x0);
	io_out(PIC1_DATA_PORT, 0x1);
	io_out(PIC1_DATA_PORT, 0x1);
	io_out(PIC1_DATA_PORT, 0xFF);
	io_out(PIC1_DATA_PORT, 0xFF);

	idt_set((sizeof(IdtEntry) * IDT_MAX_SIZE) - 1, (uint32_t)idt_table);
	enable_interrupts();
}
