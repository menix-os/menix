// Thread creation and deletion functions

#include <menix/common.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/sch/thread.h>
#include <menix/util/spin.h>

SpinLock thread_lock = spin_new();
static usize tid_counter = 0;

void thread_set_errno(usize errno)
{
	Thread* t = arch_current_cpu()->thread;
	if (t)
		t->errno = errno;
}

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

void thread_setup_execve(Thread* target, VirtAddr start, char** argv, char** envp)
{
	thread_setup(target, start, true, 0);

	Process* proc = target->parent;

	// Stack layout starting at CONFIG_user_stack_addr:
	// envp data
	// argv data
	// - 16 byte alignment -
	// auxval terminator
	// auxvals
	// 0
	// envp[0..n]
	// 0
	// argv[0..n]
	// argc

	void* stack = (void*)(target->stack + CONFIG_user_stack_size + pm_get_phys_base());

	// Copy envp onto the stack.
	usize num_envp;
	for (num_envp = 0; envp[num_envp] != NULL; num_envp++)
	{
		const usize envp_strlen = strlen(envp[num_envp]) + 1;
		stack -= envp_strlen;
		memcpy(stack, envp[num_envp], envp_strlen);
	}
	VirtAddr envp_addr =
		proc->stack_top + (target->stack + CONFIG_user_stack_size) - ((PhysAddr)(stack - pm_get_phys_base()));

	// Copy argv onto the stack.
	usize num_argv;
	for (num_argv = 0; argv[num_argv] != NULL; num_argv++)
	{
		const usize argv_strlen = strlen(argv[num_argv]) + 1;
		stack -= argv_strlen;
		memcpy(stack, argv[num_argv], argv_strlen);
	}
	VirtAddr argv_addr =
		proc->stack_top + (target->stack + CONFIG_user_stack_size) - ((PhysAddr)(stack - pm_get_phys_base()));

	// We are now working with pointer-width granularity.
	// Align the stack to a multiple of 16 so it can properly hold pointer data.
	usize* sized_stack = (usize*)ALIGN_DOWN((VirtAddr)stack, 16);

	// auxval terminator
	*(--sized_stack) = 0;
	*(--sized_stack) = 0;

	// TODO: auxvals

	// Set each envp pointer.
	*(--sized_stack) = 0;		// End of envp (== NULL).
	sized_stack -= num_envp;	// Make room for all envp entries.
	usize offset = 0;
	for (isize i = num_envp - 1; i >= 0; i--)
	{
		if (i != num_envp - 1)
			offset += strlen(envp[i + 1]) + 1;
		sized_stack[i] = envp_addr + offset;
	}

	// Set each argv pointer.
	*(--sized_stack) = 0;		// End of argv (== NULL).
	sized_stack -= num_argv;	// Make room for all argv entries.
	offset = 0;
	for (isize i = num_argv - 1; i >= 0; i--)
	{
		if (i != num_argv - 1)
			offset += strlen(argv[i + 1]) + 1;
		sized_stack[i] = argv_addr + offset;
	}

	// Set argc.
	*(--sized_stack) = num_argv;

	// Update stack start.
	target->registers.rsp -=
		(target->stack + CONFIG_user_stack_size) - (((PhysAddr)sized_stack) - (PhysAddr)pm_get_phys_base());
	proc->stack_top -= CONFIG_user_stack_size;
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
