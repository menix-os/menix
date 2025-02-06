// Process creation

#include <menix/common.h>
#include <menix/io/terminal.h>
#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/system/arch.h>
#include <menix/system/elf.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/system/sch/thread.h>
#include <menix/util/list.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

#include <string.h>

static SpinLock proc_lock = {0};
static usize pid_counter = 0;

ProcessList dead_processes;

Process* proc_create(const char* name, ProcessState state, bool is_user, Process* parent)
{
	spin_lock(&proc_lock);

	print_log("process: Creating new process \"%s\" (%s)\n", name, is_user ? "User" : "Kernel");

	Process* proc = kzalloc(sizeof(Process));
	strncpy(proc->name, name, sizeof(proc->name));

	proc->state = state;
	proc->id = pid_counter++;

	proc_setup(proc, is_user);

	// If we have a parent, copy over the parent's attributes.
	if (parent != NULL)
	{
		proc->parent = parent;
		proc->map_base = parent->map_base;
	}
	else
	{
		proc->map_base = VM_USER_MAP_BASE;
	}

	list_new(proc->threads, 0);
	list_new(proc->children, 0);

	sch_add_process(&proc_list, proc);

	spin_unlock(&proc_lock);
	return proc;
}

bool proc_create_elf(const char* name, const void* data, usize length, char** argv, char** envp)
{
	if (name == NULL)
	{
		print_error("process: Unable to load ELF: Name can't be NULL.\n");
		return false;
	}
	if (data == NULL)
	{
		print_error("process: Unable to load ELF: Data is NULL.\n");
		return false;
	}

	if (length == 0)
	{
		print_error("process: Unable to load ELF: Length is 0.\n");
		return false;
	}

	// Create a new page map for the process.
	PageMap* map = vm_page_map_new();

	// Load the executable into the new page map.
	ElfInfo info = {0};
	if (elf_load(map, data, length, &info) == false)
	{
		print_log("process: Unable to load \"%s\"\n", name);
		vm_page_map_destroy(map);
		return false;
	}

	VirtAddr entry_point = info.at_entry;

	// Use the current thread as parent.
	Process* proc = proc_create(name, ProcessState_Ready, true, arch_current_cpu()->thread->parent);

	proc->page_map = map;
	proc->elf_info = info;

	Thread* thread = thread_create(proc);
	thread_setup(thread, entry_point, argv, envp, true);

	return true;
}

void proc_kill(Process* proc, bool is_crash)
{
	kassert(proc != NULL, "No process given to kill!");
	print_log("process: Killing PID %zu\n", proc->id);

	// If the process being killed is the currently running process.
	bool is_suicide = false;
	if (arch_current_cpu()->thread->parent == proc)
		is_suicide = true;

	// Hand the threads over to the hangman.
	spin_lock_scope(&proc_lock, {
		for (usize i = 0; i < proc->threads.length; i++)
		{
			sch_remove_thread(&thread_list, proc->threads.items[i]);
			sch_add_thread(&hanging_thread_list, proc->threads.items[i]);
		}
	});

	// Remove the process from its parent.
	if (proc->parent != NULL)
	{
		usize idx;
		if (list_find(&proc->parent->children, idx, proc))
			list_pop(&proc->parent->children, idx);
	}

	// Remove the process from the scheduler.
	spin_lock_scope(&proc_lock, { sch_remove_process(&proc_list, proc); });

	// Attach orphaned processes to init (PID 1).
	Process* init = sch_id_to_process(1);
	list_iter(&proc->children, iter)
	{
		(*iter)->parent = init;
		list_push(&init->children, *iter);
	}

	if (is_suicide && !is_crash)
	{
		// TODO
	}
	else
	{
		proc->state = ProcessState_Dead;
	}

	list_push(&dead_processes, proc);
	list_free(&proc->children);
	list_free(&proc->threads);

	spin_lock_scope(&proc_lock, {
		sch_remove_process(&proc_list, proc);
		sch_add_process(&hanging_proc_list, proc);
	});

	if (is_suicide)
		arch_current_cpu()->thread = NULL;
}

void proc_setup(Process* proc, bool is_user)
{
	if (is_user)
		proc->page_map = vm_page_map_new();
	else
		proc->page_map = vm_kernel_map;
}

void proc_destroy(Process* proc)
{
	if (arch_current_cpu()->thread == NULL)
		vm_set_page_map(vm_kernel_map);
	// TODO: Crashes
	vm_page_map_destroy(proc->page_map);
}
