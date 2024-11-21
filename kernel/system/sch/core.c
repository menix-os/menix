// Cross-platform scheduler implementations

#include <menix/common.h>
#include <menix/system/boot.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/sch/thread.h>
#include <menix/util/spin.h>

Process* proc_list = NULL;
Process* hanging_proc_list = NULL;

Thread* thread_list = NULL;
Thread* hanging_thread_list = NULL;
Thread* sleeping_thread_list = NULL;

void sch_init(BootInfo* info)
{
	// Create the first process for kernel tasks (PID 0).
	Process* kernel_proc = proc_create("kernel", ProcessState_Ready, false, NULL);
	Thread* kernel_thread = thread_create(kernel_proc);
	thread_setup(kernel_thread, (VirtAddr)kernel_main, false, 0);
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
		if (spin_try_lock(&cur->lock))
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

Context* sch_reschedule(Context* context)
{
	vm_set_page_map(vm_kernel_map);

	Cpu* cur = arch_current_cpu();
	cur->ticks_active++;

	if (spin_try_lock(&rope_lock))
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
		spin_unlock(&rope_lock);
	}

	Thread* running = cur->thread;

	// Update the state of the currently running thread.
	if (running != NULL)
	{
		// Save the current context.
		running->registers = *context;
		running->stack = cur->user_stack;
		running->kernel_stack = cur->kernel_stack;

		sch_arch_save(cur, running);
		if (running->state == ThreadState_Running)
			running->state = ThreadState_Ready;
	}

	// Grab the next thread.
	running = sch_next(running);

	if (cur->thread != NULL)
	{
		// The old thread is now free.
		spin_unlock(&cur->thread->lock);
	}

	// If there are no more threads to run, wait until an interrupt happens.
	if (running == NULL)
	{
		cur->thread = NULL;
		sch_arch_stop();
	}

	// Update CPU information.
	cur->user_stack = running->stack;
	cur->kernel_stack = running->kernel_stack;
	running->state = ThreadState_Running;
	cur->thread = running;
	sch_arch_update(cur, running);

	// Load new page map.
	vm_set_page_map(running->parent->page_map);
	return &running->registers;
}
