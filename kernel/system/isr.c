// Interrupt service routines

#include <menix/common.h>
#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/interrupts.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/util/log.h>

Context* isr_handler(Context* regs)
{
	const Cpu* current = arch_current_cpu();
	// If we have a handler for this interrupt, call it.
	if (regs->isr < ARRAY_SIZE(current->irq_handlers) && current->irq_handlers[regs->isr])
	{
		Context* result = current->irq_handlers[regs->isr](regs);
		return result;
	}

	// If unhandled and caused by the user, terminate the process with SIGILL.
	if (current->thread && current->thread->is_user)
	{
		Process* proc = current->thread->parent;
		print_log("Unhandled interrupt %zu caused by user program! Terminating PID %i!\n", regs->isr, proc->id);
		arch_dump_registers(regs);
		proc_kill(proc, true);
		return sch_reschedule(regs);
	}

	// Disable spinlocks so we have a chance of displaying a message.
	// In this state everything could be broken anyways.
	spin_use(false);

	// Exception was not caused by the user and is not handled, abort.
	print_log("Unhandled interrupt %zu in kernel mode!\n", regs->isr);

	ktrace(regs);
	kabort();
}

void isr_register_handler(usize cpu, usize idx, InterruptFn handler)
{
	asm_interrupt_disable();
	if (cpu >= ARRAY_SIZE(per_cpu_data) && idx >= ARRAY_SIZE(per_cpu_data[cpu].irq_handlers))
	{
		arch_log("Failed to register a handler for ISR %zu! Out of bounds.\n", idx);
		return;
	}

	per_cpu_data[cpu].irq_handlers[idx] = handler;
	arch_log("Registered handler 0x%p for interrupt %zu on CPU %zu!\n", handler, idx, cpu);
	asm_interrupt_enable();
}
