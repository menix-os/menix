// x86 scheduling routines

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/thread/scheduler.h>

void scheduler_invoke()
{
	// Disable other interrupts so we don't accidentally get rescheduled.
	asm_interrupt_disable();

	// Force a software interrupt.
	asm_int(INT_TIMER);
}
