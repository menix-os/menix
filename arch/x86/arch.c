//? x86 platform initialization

#include <menix/arch.h>
#include <menix/log.h>
#include <menix/serial.h>

#include <arch_bits.h>
#include <gdt.h>
#include <idt.h>

void arch_init()
{
	// Install the Global Descriptor Table.
	gdt_init();
	// Install the Interrupt Descriptor Table.
	idt_init();
	// Init console output.
	serial_initialize();
}

ATTR(noreturn) void interrupt_error(void)
{
	kmesg(LOG_ERR, "\nunhandled kernel error!");

	// Stop the kernel.
	asm volatile("cli\nhlt");
	while (1)
		;
}
