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

extern void scheduler_context_switch(CpuRegisters* registers);

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

static void kill_dying_threads()
{
	if (spin_acquire(&rope_lock))
	{
		Thread* this = hanging_thread_list;
		while (this)
		{
			scheduler_remove_thread(&hanging_thread_list, this);
			Thread* next = this->next;
			thread_destroy(this);
			kfree(this);
			this = next;
		}
		spin_free(&rope_lock);
	}
}

static void kill_dying_processes()
{
	if (spin_acquire(&rope_lock))
	{
		Process* this = hanging_process_list;
		while (this)
		{
			scheduler_remove_process(&hanging_process_list, this);
			Process* next = this->next;
			process_destroy(this);
			kfree(this);
			this = next;
		}
		spin_free(&rope_lock);
	}
}

void scheduler_reschedule(CpuRegisters* regs)
{
	asm_interrupt_disable();

	vm_set_page_map(vm_get_kernel_map());
	arch_current_cpu()->ticks_active++;
	// timer_stop_sched();

	kill_dying_threads();
	kill_dying_processes();

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
			arch_current_cpu()->fpu_save(running->saved_fpu);
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
		asm_interrupt_enable();
		while (true)
			asm volatile("hlt");
	}

	arch_current_cpu()->thread = running;
	arch_current_cpu()->tss.rsp0 = running->kernel_stack;

	scheduler_context_switch(&running->registers);
}
