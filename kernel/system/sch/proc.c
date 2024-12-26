// Process creation

#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/fs/fd.h>
#include <menix/fs/vfs.h>
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

	spin_unlock(&proc_lock);
	return proc;
}

bool proc_create_elf(const char* name, const char* path, char** argv, char** envp, bool is_user)
{
	kassert(path != NULL, "Path can't be null!");

	// Open the file and ensure it's there.
	VfsNode* node = vfs_get_node(vfs_get_root(), path, true);
	if (node == NULL)
	{
		print_log("process: Unable to read \"%s\": %zu\n", path, thread_errno);
		return false;
	}

	// Create a new page map for the process.
	PageMap* map = vm_page_map_new();

	// Load the executable into the new page map.
	ElfInfo info = {0};
	if (elf_load(map, node->handle, 0, &info) == false)
	{
		print_log("process: Unable to load \"%s\": %zu\n", path, thread_errno);
		vm_page_map_destroy(map);
		return false;
	}

	VirtAddr entry_point = info.at_entry;

	// If an interpreter was requested, load it at the configured base address.
	if (info.ld_path != NULL)
	{
		VfsNode* interp = vfs_get_node(vfs_get_root(), info.ld_path, true);
		if (interp == NULL)
		{
			print_log("process: Unable to load interpreter \"%s\" for \"%s\": %zu\n", info.ld_path, path, thread_errno);
			vm_page_map_destroy(map);
			return false;
		}

		ElfInfo interp_info = {0};
		if (elf_load(map, interp->handle, CONFIG_user_interp_base, &interp_info) == false)
		{
			print_log("process: Unable to load interpreter \"%s\" for \"%s\": %zu\n", info.ld_path, path, thread_errno);
			vm_page_map_destroy(map);
			return false;
		}

		// If loading the interpreter was succesful, overwrite the entry point.
		entry_point = interp_info.at_entry;
	}

	// If no name is given, use the name of the executable.
	if (name == NULL)
		name = node->name;

	// Use the current thread as parent.
	Process* proc = proc_create(name, ProcessState_Ready, is_user, arch_current_cpu()->thread->parent);

	proc->page_map = map;
	proc->working_dir = node->parent;
	proc->map_base = CONFIG_user_map_base;

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

	Thread* thread = thread_create(proc);
	proc->elf_info = info;
	thread_setup(thread, entry_point, argv, envp, is_user);

	vm_set_page_map(map);

	return true;
}

usize proc_fork(Process* proc, Thread* thread)
{
	spin_lock(&proc_lock);

	Process* fork = kzalloc(sizeof(Process));
	strncpy(fork->name, proc->name, sizeof(proc->name));

	// Copy relevant process info.
	fork->map_base = proc->map_base;
	fork->id = pid_counter++;
	fork->permissions = proc->permissions;
	fork->parent = proc;
	fork->working_dir = proc->working_dir;

	fork->page_map = vm_page_map_fork(proc->page_map);

	list_new(fork->children, 0);
	list_new(fork->threads, 0);

	// Link the newly forked process to the parent.
	list_push(&proc->children, fork);

	for (usize i = 0; i < OPEN_MAX; i++)
	{
		if (proc->file_descs[i] == NULL)
			continue;

		// TODO: Duplicate FDs.
		fork->file_descs[i] = proc->file_descs[i];
	}

	sch_add_process(&proc_list, fork);
	thread_fork(fork, thread);

	fork->state = ProcessState_Ready;
	spin_unlock(&proc_lock);

	print_log("process: Forked process \"%s\", new pid %zu\n", proc->name, fork->id);
	return fork->id;
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

	spin_lock_scope(&proc_lock, {
		sch_remove_process(&proc_list, proc);
		sch_add_process(&hanging_proc_list, proc);
	});

	if (is_suicide)
		arch_current_cpu()->thread = NULL;
}

FileDescriptor* proc_fd_to_ptr(Process* process, usize fd)
{
	kassert(process != NULL, "No process specified! This is a kernel bug.");

	// Check if fd is within bounds.
	if (fd >= OPEN_MAX)
		return NULL;

	FileDescriptor* file_desc = NULL;
	spin_lock_scope(&process->fd_lock, {
		file_desc = process->file_descs[fd];
		if (file_desc == NULL)
			break;
	});
	return file_desc;
}

void proc_setup(Process* proc, bool is_user)
{
	if (is_user)
		proc->page_map = vm_page_map_new();
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
	// TODO: Crashes
	vm_page_map_destroy(proc->page_map);
}
