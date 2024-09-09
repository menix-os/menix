// Interrupt handlers (called by ASM stubs)

#include <menix/common.h>
#include <menix/log.h>
#include <menix/sys/syscall.h>
#include <menix/thread/process.h>

#include <idt.h>
#include <interrupts.h>

static const char* exception_names[0x20] = {
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

static void interrupt_ud_handler(CpuRegisters* regs)
{
	// Make sure we're in user mode.
	kassert(regs->cs & CPL_USER, "Invalid opcode at 0x%zx on core %zu!", regs->rip, arch_current_cpu()->id);
}

void syscall_handler(CpuRegisters* regs)
{
	// Save the registers.
	Cpu* const core = arch_current_cpu();
	Thread* const thread = core->thread;
	thread->registers = *regs;
	thread->stack = core->user_stack;

	// Execute the system call. For x86, this uses the SysV ABI.
	// The syscall selector also contains the return value.
	regs->rax = syscall_invoke(regs->rax, regs->rdi, regs->rsi, regs->rdx, regs->r10, regs->r8, regs->r9);
}

typedef void (*InterruptFn)(CpuRegisters* regs);

static InterruptFn exception_handlers[IDT_MAX_SIZE] = {
	[0x06] = interrupt_ud_handler,
	[0x0E] = interrupt_pf_handler,
	[0x80] = syscall_handler,
};

void interrupt_register(usize idx, void (*handler)(CpuRegisters*))
{
	asm_interrupt_disable();
	if (idx > IDT_MAX_SIZE)
		return;

	if (exception_handlers[idx] != NULL)
		return;

	exception_handlers[idx] = handler;
	asm_interrupt_enable();
}

void interrupt_handler(CpuRegisters* regs)
{
	// If caused by the user, terminate the process with SIGILL.
	if (regs->cs & CPL_USER)
	{
		// If we don't have a handler for this function, terminate the program immediately.
		if (regs->isr < ARRAY_SIZE(exception_handlers) && exception_handlers[regs->isr])
		{
			exception_handlers[regs->isr](regs);
			return;
		}

		Process* proc = arch_current_cpu()->thread->parent;
		kmesg("Unhandled exception %zu caused by user program! Terminating PID %i!\n", regs->isr, proc->id);
		// TODO: Terminate program.
		return;
	}

	kassert(regs->isr < ARRAY_SIZE(exception_handlers), "Unhandled exception %zu in kernel mode!", regs->isr);
	kassert(exception_handlers[regs->isr] != NULL, "Unhandled exception \"%s\" (%zu) in kernel mode!",
			exception_names[regs->isr], regs->isr);

	exception_handlers[regs->isr](regs);
}
