// Thread creation and deletion functions

#include <menix/common.h>
#include <menix/system/arch.h>
#include <menix/system/elf.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/sch/thread.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

SpinLock thread_lock = {0};
static usize tid_counter = 0;

void thread_set_errno(usize errno)
{
	Cpu* cur = arch_current_cpu();
	if (cur == NULL)
		return;

	Thread* t = cur->thread;
	if (t)
		t->errno = errno;
}

Thread* thread_create(Process* parent)
{
	spin_lock(&thread_lock);

	Thread* thread = kzalloc(sizeof(Thread));

	thread->id = tid_counter++;
	thread->runtime = parent->runtime;
	thread->parent = parent;

	thread->next = NULL;
	thread->lock = (SpinLock) {0};
	thread->state = ThreadState_Ready;

	// Register thread.
	list_push(&parent->threads, thread);
	sch_add_thread(&thread_list, thread);

	spin_unlock(&thread_lock);
	return thread;
}

void thread_execve(Thread* target, VirtAddr start, char** argv, char** envp, bool is_user)
{
	kassert(argv != NULL, "argv can't be null!");
	kassert(envp != NULL, "envp can't be null!");
	thread_setup(target, start, is_user, 0);

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

	const usize foreign_pages = CONFIG_user_stack_size / vm_get_page_size(VMLevel_0);
	void* foreign = vm_map_foreign(proc->page_map, target->stack, foreign_pages);
	void* stack = foreign + CONFIG_user_stack_size;

	// Copy envp onto the stack.
	usize num_envp;
	for (num_envp = 0; envp[num_envp] != NULL; num_envp++)
	{
		const usize envp_strlen = strlen(envp[num_envp]) + 1;
		stack -= envp_strlen;
		memcpy(stack, envp[num_envp], envp_strlen);
	}
	VirtAddr envp_addr = stack - foreign + target->stack;

	// Copy argv onto the stack.
	usize num_argv;
	for (num_argv = 0; argv[num_argv] != NULL; num_argv++)
	{
		const usize argv_strlen = strlen(argv[num_argv]) + 1;
		stack -= argv_strlen;
		memcpy(stack, argv[num_argv], argv_strlen);
	}
	VirtAddr argv_addr = stack - foreign + target->stack;

	// We are now working with pointer-width granularity.
	// Align the stack to a multiple of 16 so it can properly hold pointer data.
	usize* sized_stack = (usize*)ALIGN_DOWN((VirtAddr)stack, 16);

	// Align the stack if argc + argv + envp does not add up to 16 byte alignment.
	if ((1 + num_argv + num_envp) % 2 == 1)
		*(--sized_stack) = 0;

	// Auxiliary vector.
	ElfInfo* elf_info = &proc->elf_info;
	// Terminator
	*(--sized_stack) = 0;
	*(--sized_stack) = 0;
	// AT_SECURE
	*(--sized_stack) = 0;
	*(--sized_stack) = 23;
	// AT_PHDR
	*(--sized_stack) = elf_info->at_phdr;
	*(--sized_stack) = AT_PHDR;
	// AT_PHNUM
	*(--sized_stack) = elf_info->at_phnum;
	*(--sized_stack) = AT_PHNUM;
	// AT_PHENT
	*(--sized_stack) = elf_info->at_phent;
	*(--sized_stack) = AT_PHENT;
	// AT_ENTRY
	*(--sized_stack) = elf_info->at_entry;
	*(--sized_stack) = AT_ENTRY;

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
#ifdef CONFIG_arch_x86_64
	target->registers.rsp = (void*)sized_stack - foreign + target->stack;
#elif defined(CONFIG_arch_riscv64)
	target->registers.x2 = (void*)sized_stack - foreign + target->stack;
#endif

	vm_unmap_foreign(foreign, foreign_pages);

	kassert(target->registers.rsp % 16 == 0, "Stack is misaligned: 0x%p!", target->registers.rsp);
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
