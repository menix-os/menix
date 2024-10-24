// Thread creation and deletion functions

#include <menix/common.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/sch/thread.h>
#include <menix/util/spin.h>

SpinLock thread_lock = spin_new();
static usize tid_counter = 0;

void thread_create(Process* parent, VirtAddr start, bool is_user)
{
	spin_acquire_force(&thread_lock);

	Thread* thread = kzalloc(sizeof(Thread));

	thread->id = tid_counter++;
	thread->runtime = parent->runtime;
	thread->parent = parent;

	thread_setup(thread, start, is_user, 0);

	thread->next = NULL;
	thread->lock = spin_new();
	thread->state = ThreadState_Ready;

	// Register thread.
	list_push(&parent->threads, thread);
	sch_add_thread(&thread_list, thread);

	spin_free(&thread_lock);
}

void thread_execve(Process* parent, Thread* target, VirtAddr start, char** argv, char** envp)
{
}

void thread_sleep(Thread* target, usize nanoseconds)
{
}

void thread_fork(Process* parent, Thread* target)
{
}

void thread_hang(Thread* victim, bool reschedule)
{
}

void thread_kill(Thread* victim)
{
}
