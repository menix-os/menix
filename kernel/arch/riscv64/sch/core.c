// riscv64 specific scheduler handling.

#include <menix/system/arch.h>
#include <menix/system/sch/scheduler.h>
#include <menix/util/log.h>

void sch_arch_invoke()
{
	// Make sure interrupts are enabled.
	asm_interrupt_enable();

	// Force a software interrupt.
	// TODO
}

void sch_arch_save(CpuInfo* core, Thread* thread)
{
	// TODO
}

void sch_arch_update(CpuInfo* core, Thread* next)
{
	// TODO
}

ATTR(noreturn) void sch_arch_stop()
{
	asm_interrupt_enable();
	while (true)
		asm_halt();
}
