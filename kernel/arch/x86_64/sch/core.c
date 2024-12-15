// x86_64 specific scheduler handling.

#include <menix/system/arch.h>
#include <menix/system/sch/scheduler.h>

#include <apic.h>

void sch_invoke()
{
	// Make sure interrupts are enabled.
	asm_interrupt_enable();

	// Force a software interrupt.
	asm_int(INT_TIMER);
}

void sch_arch_save(Cpu* core, Thread* thread)
{
	thread->fs_base = asm_rdmsr(MSR_FS_BASE);
	thread->gs_base = asm_rdmsr(MSR_GS_BASE);

	core->fpu_save(thread->saved_fpu);
}

void sch_arch_update(Cpu* core, Thread* next)
{
	core->tss.rsp0 = next->kernel_stack;
	core->fpu_restore(next->saved_fpu);

	asm_wrmsr(MSR_FS_BASE, next->fs_base);
	asm_wrmsr(MSR_GS_BASE, next->gs_base);
}

ATTR(noreturn) void sch_arch_stop()
{
	apic_send_eoi();
	asm_interrupt_enable();
	while (true)
		asm_halt();
}
