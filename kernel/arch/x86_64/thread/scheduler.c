// x86 scheduling routines

#include <menix/common.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/thread/process.h>
#include <menix/thread/scheduler.h>
#include <menix/thread/spin.h>
#include <menix/thread/thread.h>

#include <apic.h>

SpinLock rope_lock = spin_new();
SpinLock wakeup_lock = spin_new();

extern void scheduler_context_switch(Context* registers);

void scheduler_pause()
{
	// Disable interrupts so the scheduler doesn't get triggered by the timer interrupt.
	asm_interrupt_disable();
}

void scheduler_invoke()
{
	asm_interrupt_enable();

	// Force a software interrupt.
	asm_int(INT_TIMER);
}

void scheduler_reschedule(Context* regs)
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
			scheduler_remove_thread(&hanging_thread_list, thread);
			Thread* next = thread->next;
			thread_destroy(thread);
			kfree(thread);
			thread = next;
		}

		// Kill dying processes.
		Process* proc = hanging_process_list;
		while (proc)
		{
			scheduler_remove_process(&hanging_process_list, proc);
			Process* next = proc->next;
			process_destroy(proc);
			kfree(proc);
			proc = next;
		}
		spin_free(&rope_lock);
	}

	Thread* running = arch_current_cpu()->thread;

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
			running->registers = *regs;
			running->fs_base = asm_rdmsr(MSR_FS_BASE);
			running->gs_base = asm_rdmsr(MSR_GS_BASE);
			// TODO: Causes a GPF
			// arch_current_cpu()->fpu_save(running->saved_fpu);
			running->stack = arch_current_cpu()->user_stack;
			running->kernel_stack = arch_current_cpu()->kernel_stack;

			if (running->state == ThreadState_Running)
				running->state = ThreadState_Ready;
		}
	}

	// Grab the next thread.
	running = scheduler_next(running);

	// If there are no more threads to run, something went wrong.
	if (running == NULL)
	{
		apic_send_eoi();
		arch_current_cpu()->thread = NULL;
		kmesg("[Scheduler]\tNo more threads to run, halting!\n");
		asm_interrupt_enable();
		while (true)
			asm volatile("hlt");
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
	usize cr3;
	asm_get_register(cr3, cr3);
	asm_set_register(cr3, cr3);

	apic_send_eoi();

	scheduler_context_switch(&running->registers);
}
