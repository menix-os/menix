// x86 platform initialization

#include <menix/arch.h>
#include <menix/log.h>
#include <menix/serial.h>

#include <gdt.h>
#include <idt.h>
#include <interrupts.h>

void arch_early_init()
{
	// Install the Global Descriptor Table.
	gdt_init();

	// Install the Interrupt Descriptor Table.
	idt_init();

	// Init COM1 for debug (or if we don't have a frame buffer).
	serial_initialize();
}

void arch_init(BootInfo* info)
{
}
