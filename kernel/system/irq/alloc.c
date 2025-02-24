// Interrupt service routines

#include <menix/common.h>
#include <menix/memory/alloc.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/interrupts.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/sch/thread.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

static SpinLock irq_alloc_lock = {0};
static Irq irq_counter = 0;
IrqAction* irq_actions = NULL;

// Find the next free slot in the irq_actions list.
static void add_irq(IrqAction* target)
{
	if (target == NULL)
		return;

	IrqAction* cur = irq_actions;

	if (cur == NULL)
	{
		irq_actions = target;
		return;
	}

	if (cur == target)
		return;

	while (cur->next)
		cur = cur->next;

	cur->next = target;
}

bool irq_allocate(IrqHandlerFn handler, IrqHandlerFn thread_handler, IrqFlags flags, const char* name, void* data)
{
	spin_lock(&irq_alloc_lock);

	// TODO: Allocate an interrupt line on the CPU.

	IrqAction* action = kzalloc(sizeof(IrqAction));
	action->irq = irq_counter++;
	action->flags = flags;
	action->handler = handler;
	action->worker = thread_handler;
	action->name = name;
	action->context = data;

	action->thread = thread_new(proc_kernel);
	thread_arch_setup(action->thread, 0, false, 0);

	add_irq(action);

	spin_unlock(&irq_alloc_lock);
	return true;
}
