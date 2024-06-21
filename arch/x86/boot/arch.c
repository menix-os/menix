//? x86 platform initialization

#include <menix/arch.h>
#include <menix/serial.h>
#include <menix/stdio.h>

#include <arch_bits.h>
#include <gdt.h>
#include <idt.h>
#include <multiboot.h>

void arch_init()
{
	// Install the Global Descriptor Table.
	gdt_set(sizeof(gdt_table), (uint32_t)gdt_table);
	// Install the Interrupt Descriptor Table.
	idt_init();

	// Init console output.
	serial_initialize();
}

void interrupt_disable()
{
	asm volatile("cli");
}

void interrupt_enable()
{
	asm volatile("sti");
}

ATTR(noreturn) void interrupt_error(void)
{
	printf("\nunhandled kernel error!");

	// Stop the kernel.
	asm volatile("cli\n"
				 "hlt");
	while (1)
	{
	}
}

uint8_t read8(uint16_t port)
{
	uint8_t result;
	asm volatile("movw %0, %%dx" ::"r"(port));		// Move "port" into dx (mandatory).
	asm volatile("inb %%dx, %0" : "=r"(result));	// Get byte from "port" and store in "result".
	return result;
}

uint16_t read16(uint16_t port)
{
	uint16_t result;
	asm volatile("movw %0, %%dx" ::"r"(port));		// Move "port" into dx (mandatory).
	asm volatile("inw %%dx, %0" : "=r"(result));	// Get byte from "port" and store in "result".
	return result;
}

uint32_t read32(uint16_t port)
{
	uint32_t result;
	asm volatile("movw %0, %%dx" ::"r"(port));		// Move "port" into dx (mandatory).
	asm volatile("inl %%dx, %0" : "=r"(result));	// Get byte from "port" and store in "result".
	return result;
}

void write8(uint16_t port, uint8_t value)
{
	asm volatile("movw %0, %%dx" ::"r"(port));	   // Move "port" into dx (mandatory).
	asm volatile("outb %0, %%dx" ::"r"(value));	   // Write byte to "port".
}

void write16(uint16_t port, uint16_t value)
{
	asm volatile("movw %0, %%dx" ::"r"(port));	   // Move "port" into dx (mandatory).
	asm volatile("outw %0, %%dx" ::"r"(value));	   // Write byte to "port".
}

void write32(uint16_t port, uint32_t value)
{
	asm volatile("movw %0, %%dx" ::"r"(port));	   // Move "port" into dx (mandatory).
	asm volatile("outl %0, %%dx" ::"r"(value));	   // Write byte to "port".
}
