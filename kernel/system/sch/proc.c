// Process creation

#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/fs/fd.h>
#include <menix/fs/vfs.h>
#include <menix/io/terminal.h>
#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/system/elf.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>
#include <menix/util/list.h>
#include <menix/util/spin.h>

#include <string.h>

static SpinLock proc_lock = spin_new();
static usize pid_counter = 0;

ProcessList dead_processes;

Process* proc_create(char* name, ProcessState state, VirtAddr ip, bool is_user, Process* parent)
{
	spin_acquire_force(&proc_lock);

	Process* proc = kzalloc(sizeof(Process));
	strncpy(proc->name, name, sizeof(proc->name));

	proc->state = state;
	proc->id = pid_counter++;

	proc_setup(proc, is_user);

	proc->working_dir = vfs_get_root();

	// If we have a parent, copy over the parent's attributes.
	if (parent != NULL)
	{
		if (parent->working_dir != NULL)
			proc->working_dir = parent->working_dir;
		proc->permissions = parent->permissions;
		proc->parent = parent;
		proc->map_base = parent->map_base;
	}
	else
	{
		proc->permissions = S_IWGRP | S_IWOTH;
		proc->map_base = CONFIG_vm_map_base;
	}

	list_new(proc->threads, 0);
	list_new(proc->children, 0);

	sch_add_process(&proc_list, proc);
	thread_create(proc, ip, is_user);

	spin_free(&proc_lock);
	return proc;
}

bool proc_create_elf(char* name, ProcessState state, Process* parent, const char* path)
{
	spin_acquire_force(&proc_lock);

	Process* proc = kzalloc(sizeof(Process));
	strncpy(proc->name, name, sizeof(proc->name));

	proc->state = state;
	proc->id = pid_counter++;

	proc_setup(proc, true);

	proc->working_dir = vfs_get_root();
	proc->stack_top = CONFIG_user_stack_base;

	// If we have a parent, copy over the parent's attributes.
	if (parent != NULL)
	{
		if (parent->working_dir != NULL)
			proc->working_dir = parent->working_dir;
		proc->permissions = parent->permissions;
		proc->parent = parent;
		proc->map_base = parent->map_base;
	}
	else
	{
		proc->permissions = S_IWGRP | S_IWOTH;
		proc->map_base = CONFIG_vm_map_base;
	}

	// Open the file and ensure it's there.
	VfsNode* node = vfs_get_node(vfs_get_root(), path, true);
	if (node == NULL)
	{
		proc_log("Unable to read \"%s\": %zu\n", path, thread_errno);
		spin_free(&proc_lock);
		return false;
	}

	// Load the executable into the new page map.
	ElfInfo info = {0};
	if (elf_load(proc->page_map, node->handle, 0, &info) == false)
	{
		proc_log("Unable to load \"%s\": %zu\n", path, thread_errno);
		return false;
	}

	// If an interpreter was requested, load it at the configured base address.
	if (info.ld_path != NULL)
	{
		VfsNode* interp = vfs_get_node(vfs_get_root(), info.ld_path, true);
		if (interp == NULL)
		{
			proc_log("Unable to load interpreter \"%s\" for \"%s\": %zu\n", info.ld_path, path, thread_errno);
			return false;
		}

		ElfInfo interp_info = {0};
		if (elf_load(proc->page_map, interp->handle, CONFIG_user_interp_base, &interp_info) == false)
		{
			proc_log("Unable to load interpreter \"%s\" for \"%s\": %zu\n", info.ld_path, path, thread_errno);
			return false;
		}
	}

	list_new(proc->threads, 0);
	list_new(proc->children, 0);

	// TODO: Make a proper IO interface
	FileDescriptor* desc = kzalloc(sizeof(FileDescriptor));
	VfsNode* terminal = terminal_get_active_node();
	if (terminal)
	{
		desc->handle = terminal->handle;
		proc->file_descs[0] = desc;
		proc->file_descs[1] = desc;
		proc->file_descs[2] = desc;
	}

	sch_add_process(&proc_list, proc);
	thread_create(proc, info.entry_point, true);

	spin_free(&proc_lock);
	return proc;
}

bool proc_execve(const char* path, char** argv, char** envp)
{
	spin_acquire_force(&proc_lock);
	sch_pause();

	// Use the current thread's structures.
	Thread* thread = arch_current_cpu()->thread;
	Process* proc = thread->parent;

	// Open the file and ensure it's there.
	VfsNode* node = vfs_get_node(vfs_get_root(), path, true);
	if (node == NULL)
	{
		proc_log("Unable to read \"%s\": %zu\n", path, thread_errno);
		spin_free(&proc_lock);
		return false;
	}

	proc_setup(proc, true);

	// Create a new page map for the process.
	PageMap* map = vm_page_map_new(VMLevel_0);

	// Load the executable into the new page map.
	ElfInfo info = {0};
	if (elf_load(map, node->handle, 0, &info) == false)
	{
		proc_log("Unable to load \"%s\": %zu\n", path, thread_errno);
		vm_page_map_destroy(map);
		spin_free(&proc_lock);
		return false;
	}

	// If an interpreter was requested, load it at the configured base address.
	if (info.ld_path != NULL)
	{
		VfsNode* interp = vfs_get_node(vfs_get_root(), info.ld_path, true);
		if (interp == NULL)
		{
			proc_log("Unable to load interpreter \"%s\" for \"%s\": %zu\n", info.ld_path, path, thread_errno);
			vm_page_map_destroy(map);
			spin_free(&proc_lock);
			return false;
		}

		ElfInfo interp_info = {0};
		if (elf_load(map, interp->handle, CONFIG_user_interp_base, &interp_info) == false)
		{
			proc_log("Unable to load interpreter \"%s\" for \"%s\": %zu\n", info.ld_path, path, thread_errno);
			vm_page_map_destroy(map);
			spin_free(&proc_lock);
			return false;
		}
	}

	// If loading was successful, set the new map.
	thread->parent->page_map = map;

	// Set CWD.
	thread->parent->working_dir = node->parent;

	proc->map_base = CONFIG_vm_map_base;
	proc->stack_top = CONFIG_user_stack_base;
	proc->state = ProcessState_Ready;

	// Map the process stack. Subtract size from the start since stack grows down.
	vm_map(map, proc->stack_top - CONFIG_user_stack_size, CONFIG_user_stack_size,
		   VMProt_Write | VMProt_Read | VMProt_Execute, 0, VMLevel_0);

	arch_current_cpu()->user_stack = proc->stack_top;

	thread_execve(proc, thread, info.entry_point, argv, envp);

	vm_set_page_map(map);
	spin_free(&proc_lock);

	// Run the scheduler.
	sch_invoke();

	return false;
}

usize proc_fork(Process* proc, Thread* thread)
{
	spin_acquire_force(&proc_lock);

	Process* fork = kzalloc(sizeof(Process));
	strncpy(fork->name, proc->name, sizeof(proc->name));

	// Copy relevant process info.
	fork->map_base = proc->map_base;
	fork->stack_top = proc->stack_top;
	fork->id = pid_counter++;
	fork->permissions = proc->permissions;
	fork->parent = proc;
	fork->working_dir = proc->working_dir;
	fork->next = NULL;

	list_new(fork->children, 0);
	list_new(fork->threads, 0);

	// Link the newly forked process to the parent.
	list_push(&proc->children, fork);

	for (usize i = 0; i < OPEN_MAX; i++)
	{
		if (proc->file_descs[i] == NULL)
			continue;

		// TODO: Duplicate FDs.
	}

	sch_add_process(&proc_list, fork);
	thread_fork(fork, thread);

	fork->state = ProcessState_Ready;
	spin_free(&proc_lock);

	return fork->id;
}

void proc_kill(Process* proc, bool is_crash)
{
	sch_pause();
	if (proc->id <= 1)
		kmesg("[WARNING]\tKilling init or kernel process!\n");

	// If the process being killed is the currently running process.
	bool is_suicide = false;
	if (arch_current_cpu()->thread->parent == proc)
		is_suicide = true;

	// Hand the threads over to the hangman.
	spin_lock(&proc_lock, {
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

	Process* init = proc_list->next;

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
		proc->return_code = -1;
		proc->state = ProcessState_Dead;
	}

	for (usize i = 0; i < OPEN_MAX; i++)
	{
		if (proc->file_descs[i] == NULL)
			continue;
		// TODO: fdnum_close(proc, i);
	}

	list_push(&dead_processes, proc);
	list_free(&proc->children);
	list_free(&proc->threads);

	spin_lock(&proc_lock, {
		sch_remove_process(&proc_list, proc);
		sch_add_process(&hanging_proc_list, proc);
	});

	if (is_suicide)
		arch_current_cpu()->thread = NULL;

	sch_invoke();
}

FileDescriptor* proc_fd_to_ptr(Process* process, usize fd)
{
	kassert(process != NULL, "No process specified! This is a kernel bug.");

	if (fd >= OPEN_MAX)
	{
		thread_set_errno(EBADF);
		return NULL;
	}

	FileDescriptor* file_desc = NULL;
	spin_lock(&process->fd_lock, {
		file_desc = process->file_descs[fd];
		if (file_desc == NULL)
		{
			thread_set_errno(EBADF);
			break;
		}
	});
	return file_desc;
}

void proc_setup(Process* proc, bool is_user)
{
	if (is_user)
		proc->page_map = vm_page_map_new(VMLevel_0);
	else
		proc->page_map = vm_kernel_map;
}

void proc_fork_context(Process* fork, Process* source)
{
	fork->page_map = vm_page_map_fork(source->page_map);
	// TODO
}

void proc_destroy(Process* proc)
{
	if (arch_current_cpu()->thread == NULL)
		vm_set_page_map(vm_kernel_map);
	vm_page_map_destroy(proc->page_map);
}
