// Cross-platform scheduler implementations

#include <menix/common.h>
#include <menix/system/boot.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/sch/thread.h>
#include <menix/util/spin.h>

#ifdef CONFIG_arch_x86_64
#include <apic.h>
#endif

Process* proc_list = NULL;
Process* hanging_proc_list = NULL;

Thread* thread_list = NULL;
Thread* hanging_thread_list = NULL;
Thread* sleeping_thread_list = NULL;

void sch_init(BootInfo* info)
{
	// Create the first process for kernel tasks (PID 0).
	proc_create("kernel", ProcessState_Ready, (VirtAddr)kernel_main, false, NULL);
	sch_invoke();
}

Thread* sch_next(Thread* list)
{
	Thread* cur = NULL;

	if (list != NULL)
		cur = list;
	else
		cur = thread_list;

	// Loop until we find a thread that isn't being worked on and is ready to run.
	while (cur != NULL)
	{
		// Check if thread is ready.
		if (cur->state != ThreadState_Ready)
		{
			cur = cur->next;
			continue;
		}

		// Check if it's currently not being worked on.
		if (spin_acquire(&cur->lock))
			return cur;

		// Thread is being worked on.
		cur = cur->next;
	}

	// Nothing to schedule.
	return NULL;
}

void sch_add_thread(Thread** list, Thread* target)
{
	if (target == NULL)
		return;

	Thread* cur = *list;

	if (cur == NULL)
	{
		*list = target;
		return;
	}

	if (cur == target)
		return;

	while (cur->next)
		cur = cur->next;

	cur->next = target;
}

void sch_remove_thread(Thread** list, Thread* target)
{
	if (list == NULL && target == NULL)
		return;

	Thread* cur = *list;
	Thread* next = NULL;

	if (cur == target)
	{
		*list = cur->next;
		return;
	}

	while (cur)
	{
		next = cur->next;
		if (next == target)
		{
			cur->next = next->next;
			next->next = NULL;
		}
		cur = next;
	}
}

void sch_add_process(Process** list, Process* target)
{
	if (target == NULL)
		return;

	Process* cur = *list;

	if (cur == NULL)
	{
		*list = target;
		return;
	}

	if (cur == target)
		return;

	while (cur->next)
		cur = cur->next;

	cur->next = target;
}

void sch_remove_process(Process** list, Process* target)
{
	if (list == NULL && target == NULL)
		return;

	Process* cur = *list;
	Process* next = NULL;

	if (cur == target)
	{
		*list = cur->next;
		return;
	}

	while (cur)
	{
		next = cur->next;
		if (next == target)
		{
			cur->next = next->next;
			next->next = NULL;
		}
		cur = next;
	}
}

Thread* sch_id_to_thread(usize tid)
{
	Thread* cur = thread_list;
	while (cur)
	{
		if (cur->id == tid)
			return cur;
		cur = cur->next;
	}
	return NULL;
}

Process* sch_id_to_process(usize pid)
{
	Process* cur = proc_list;
	while (cur)
	{
		if (cur->id == pid)
			return cur;
		cur = cur->next;
	}
	return NULL;
}

SpinLock rope_lock = spin_new();
SpinLock wakeup_lock = spin_new();

// Assembly stub to return back.
extern void sch_finalize(Context* registers);

void sch_pause()
{
	// Disable interrupts so the scheduler doesn't get triggered by the timer interrupt.
	asm_interrupt_disable();
}

void sch_invoke()
{
	asm_interrupt_enable();

	// Force a software interrupt.
	asm_int(INT_TIMER);
}

void sch_reschedule(Context* regs)
{
	asm_interrupt_disable();

	vm_set_page_map(vm_kernel_map);

	Cpu* cur = arch_current_cpu();
	cur->ticks_active++;
	// timer_stop_sched();

	if (spin_acquire(&rope_lock))
	{
		// Kill dying threads.
		Thread* thread = hanging_thread_list;
		while (thread)
		{
			sch_remove_thread(&hanging_thread_list, thread);
			Thread* next = thread->next;
			thread_destroy(thread);
			kfree(thread);
			thread = next;
		}

		// Kill dying processes.
		Process* proc = hanging_proc_list;
		while (proc)
		{
			sch_remove_process(&hanging_proc_list, proc);
			Process* next = proc->next;
			proc_destroy(proc);
			kfree(proc);
			proc = next;
		}
		spin_free(&rope_lock);
	}

	Thread* running = cur->thread;

	// Update the state of the currently running thread.
	if (running != NULL)
	{
		if (running->can_exec == true)
		{
			Process* parent = running->parent;
			usize idx;
			if (list_find(&parent->threads, idx, running))
				list_pop(&parent->threads, idx);

			spin_acquire_force(&thread_lock);
		}
		else
		{
			// Save the current context.
			running->registers = *regs;
			running->stack = cur->user_stack;
			running->kernel_stack = cur->kernel_stack;

#if defined(CONFIG_arch_x86_64)
			running->fs_base = asm_rdmsr(MSR_FS_BASE);
			running->gs_base = asm_rdmsr(MSR_GS_BASE);

			// TODO: Causes a GPF
			// cur->fpu_save(running->saved_fpu);
#endif

			if (running->state == ThreadState_Running)
				running->state = ThreadState_Ready;
		}
	}

	// Grab the next thread.
	running = sch_next(running);

	// If there are no more threads to run, something went wrong.
	if (running == NULL)
	{
		apic_send_eoi();
		cur->thread = NULL;
		kmesg("[Scheduler]\tNo more threads to run, halting!\n");
		asm_interrupt_enable();
		while (true)
			asm_halt();
	}

	cur->thread = running;
	cur->tss.rsp0 = running->kernel_stack;
	cur->user_stack = running->stack;
	cur->kernel_stack = running->kernel_stack;
	cur->thread->state = ThreadState_Running;
	// TODO: Causes a GPF
	// cur->fpu_restore(running->saved_fpu);

	// Reload page map.
	vm_set_page_map(running->parent->page_map);

#if defined(CONFIG_arch_x86_64)
	apic_send_eoi();
#endif

	sch_finalize(&running->registers);
}
