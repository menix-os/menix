// Interrupt handlers (called by ASM stubs)

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/log.h>
#include <menix/thread/process.h>

#include <idt.h>
#include <interrupts.h>

static const char* exception_names[] = {
	[0x00] = "Division Error",
	[0x01] = "Debug",
	[0x02] = "Non-maskable Interrupt",
	[0x03] = "Breakpoint",
	[0x04] = "Overflow",
	[0x05] = "Bound Range Exceeded",
	[0x06] = "Invalid Opcode",
	[0x07] = "Device Not Available",
	[0x08] = "Double Fault",
	[0x09] = "Coprocessor Segment Overrun",
	[0x0A] = "Invalid TSS",
	[0x0B] = "Segment Not Present",
	[0x0C] = "Stack-Segment Fault",
	[0x0D] = "General protection Fault",
	[0x0E] = "Page Fault",
	[0x0F] = NULL,	  // Reserved
	[0x10] = "x87 Floating-Point Exception",
	[0x11] = "Alignment Check",
	[0x12] = "Machine Check",
	[0x13] = "SIMD Floating-Point Exception",
	[0x14] = "Virtualization Exception",
	[0x15] = "Control Protection Exception",
	[0x16 ... 0x1B] = NULL,	   // Reserved
	[0x1C] = "Hypervisor Injection Exception",
	[0x1D] = "VMM Communication Exception",
	[0x1E] = "Security Exception",
	[0x1F] = NULL,	  // Reserved
};

static void interrupt_breakpoint_handler(CpuRegisters* regs)
{
	asm volatile("cli");
	while (1)
		asm volatile("hlt");
}
static void interrupt_handler_invalid_opcode(CpuRegisters* regs)
{
	// Make sure we're in user mode.
	kassert(regs->cs & CPL_USER, "Invalid opcode at 0x%zx on core %zu!\n", regs->rip, arch_current_cpu()->id);
}

typedef void (*InterruptFn)(CpuRegisters* regs);
static InterruptFn exception_handlers[IDT_MAX_SIZE] = {
	[0x03] = interrupt_breakpoint_handler,
	[0x06] = interrupt_handler_invalid_opcode,
};

void interrupt_handler(CpuRegisters* regs)
{
	// If caused by the user, terminate the process with SIGILL.
	if (regs->cs & CPL_USER)
	{
		if (regs->isr < ARRAY_SIZE(exception_handlers) && exception_handlers[regs->isr])
		{
			exception_handlers[regs->isr](regs);
			return;
		}

		// If we don't have a handler for this function, terminate the program immediately.
		Process* proc = arch_current_cpu()->thread->parent;
		kmesg("Unhandled exception %zu caused by user program! Terminating PID %i!\n", regs->isr, proc->id);
		// TODO: Terminate program.
		return;
	}

	kassert(regs->isr < ARRAY_SIZE(exception_handlers), "Unhandled exception %zu in kernel mode!\n", regs->isr);
	kassert(exception_handlers[regs->isr] != NULL, "Unhandled exception \"%s\" (%zu) in kernel mode!\n",
			exception_names[regs->isr], regs->isr);

	exception_handlers[regs->isr](regs);
}
