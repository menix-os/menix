//? x86 platform initialization

#include <menix/arch.h>
#include <menix/log.h>
#include <menix/serial.h>

#include <arch_bits.h>
#include <gdt.h>
#include <idt.h>

void arch_init()
{
	// Limine handles this for us.
#ifndef CONFIG_boot_limine
	// Install the Global Descriptor Table.
	gdt_init();
#endif
	// Install the Interrupt Descriptor Table.
	idt_init();
	// Init COM1 for debug (or if we don't have a frame buffer).
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

void interrupt_syscall(CpuRegisters regs)
{
	kmesg(LOG_INFO, "Hello from system call interrupt!\n");
	kmesg(LOG_INFO, "SYSCALL(%i)\n", regs.rax);
}
