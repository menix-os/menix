// Interrupt handlers (called by ASM stubs)

#include <menix/common.h>
#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/util/log.h>

#include <apic.h>
#include <idt.h>
#include <interrupts.h>

extern bool can_smap;

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

static Context* interrupt_ud_handler(Context* regs)
{
	// Make sure we're in user mode, otherwise we have to crash.
	print_log("Invalid opcode at 0x%zx on core %zu!\n", regs->rip, arch_current_cpu()->id);
	print_log("Faulty data:");

	for (usize i = 0; i < 16; i++)
	{
		u8 instr = *(u8*)regs->rip;
		print_log(" %hhx", instr);
	}

	ktrace(regs);
	kabort();

	return regs;
}

// Handles the syscall interrupt. Also referenced by system/arch.s
Context* syscall_handler(Context* regs)
{
	// Save the registers.
	Cpu* const core = arch_current_cpu();
	Thread* const thread = core->thread;
	thread->registers = *regs;
	thread->stack = core->user_stack;
	sch_arch_save(core, thread);

	// Execute the system call. For x86_64, use the SysV kernel ABI.
	SyscallResult result = syscall_invoke(regs->rax, regs->rdi, regs->rsi, regs->rdx, regs->r10, regs->r8, regs->r9);
	regs->rax = result.value;
	regs->rdx = result.error;

	return regs;
}

// Page fault interrupt handler.
Context* interrupt_pf_handler(Context* regs);

static InterruptFn exception_handlers[IDT_MAX_SIZE] = {
	[0x06] = interrupt_ud_handler,
	[0x0E] = interrupt_pf_handler,
	[INT_TIMER] = timer_handler,
	[INT_SYSCALL] = syscall_handler,
};

void interrupt_register(usize idx, InterruptFn handler)
{
	asm_interrupt_disable();
	if (idx > IDT_MAX_SIZE)
		return;

	if (exception_handlers[idx] != NULL)
		return;

	exception_handlers[idx] = handler;
	arch_log("Registered handler 0x%p for interrupt %zu!\n", handler, idx);
	asm_interrupt_enable();
}

Context* interrupt_handler(Context* regs)
{
	// If we have a handler for this interrupt, call it.
	if (regs->isr < ARRAY_SIZE(exception_handlers) && exception_handlers[regs->isr])
	{
		return exception_handlers[regs->isr](regs);
	}

	// If unhandled and caused by the user, terminate the process with SIGILL.
	if (regs->cs & CPL_USER)
	{
		Process* proc = arch_current_cpu()->thread->parent;
		print_log("Unhandled interrupt %zu caused by user program! Terminating PID %i!\n", regs->isr, proc->id);
		arch_dump_registers(regs);

		proc_kill(proc, true);
		return regs;
	}

	// Disable spinlocks so we have a chance of displaying a message.
	// In this state everything could be broken anyways.
	spin_use(false);

	// Exception was not caused by the user and is not handled, abort.
	if (regs->isr < ARRAY_SIZE(exception_names))
		print_log("Unhandled interrupt \"%s\" (%zu) in kernel mode!\n", exception_names[regs->isr], regs->isr);
	else
		print_log("Unhandled interrupt %zu in kernel mode!\n", regs->isr);

	ktrace(regs);
	kabort();
}
