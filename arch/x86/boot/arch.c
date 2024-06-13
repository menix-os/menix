/*-------------------------
x86 platform initialization
-------------------------*/

#include <menix/arch.h>
#include <menix/serial.h>

#include <gdt.h>
#include <idt.h>
#include <multiboot.h>

void arch_init()
{
	// Install the Global Descriptor Table.
	gdt_set(sizeof(gdt_table), (uint32_t)gdt_table);
	// Install the Interrupt Descriptor Table.
	idt_init();

	// Init output.
	serial_initialize();
}
