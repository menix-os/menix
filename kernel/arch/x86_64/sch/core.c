// x86_64 specific scheduler handling.

#include <menix/system/arch.h>
#include <menix/system/sch/scheduler.h>

#include <apic.h>

ATTR(noreturn) void sch_invoke()
{
	asm_interrupt_enable();

	// Force a software interrupt.
	asm_int(INT_TIMER);

	__builtin_unreachable();
}

void sch_pause()
{
	// Disable interrupts so the scheduler doesn't get triggered by the timer interrupt.
	asm_interrupt_disable();
}

void sch_arch_save(Cpu* core, Thread* thread)
{
	thread->fs_base = asm_rdmsr(MSR_FS_BASE);
	thread->gs_base = asm_rdmsr(MSR_GS_BASE);

	core->fpu_save(thread->saved_fpu);
}

void sch_arch_update(Cpu* core, Thread* next)
{
	core->thread = next;
	core->tss.rsp0 = next->kernel_stack;
	core->user_stack = next->stack;
	core->kernel_stack = next->kernel_stack;
	core->thread->state = ThreadState_Running;
	core->fpu_restore(next->saved_fpu);
}

// Defined in system/arch.s
extern void sch_x86_finalize();

void sch_arch_finalize(Context* regs)
{
	apic_send_eoi();
	sch_x86_finalize();
}
