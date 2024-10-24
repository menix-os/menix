// Cross-platform scheduler implementations

#include <menix/common.h>
#include <menix/system/boot.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/sch/thread.h>
#include <menix/util/spin.h>

Process* process_list = NULL;
Process* hanging_process_list = NULL;

Thread* thread_list = NULL;
Thread* hanging_thread_list = NULL;
Thread* sleeping_thread_list = NULL;

void sch_init(BootInfo* info)
{
	// Create the first process for kernel tasks (PID 0).
	process_create("kernel", ProcessState_Ready, (VirtAddr)kernel_main, false, NULL);
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
	Process* cur = process_list;
	while (cur)
	{
		if (cur->id == pid)
			return cur;
		cur = cur->next;
	}
	return NULL;
}
