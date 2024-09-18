// Process creation

#include <menix/common.h>
#include <menix/fs/vfs.h>
#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/thread/elf.h>
#include <menix/thread/process.h>
#include <menix/thread/scheduler.h>
#include <menix/thread/spin.h>
#include <menix/util/list.h>

#include <errno.h>
#include <string.h>

#include "menix/fs/fd.h"
#include "menix/io/terminal.h"

static SpinLock process_lock = spin_new();
static usize pid_counter = 0;

ProcessList dead_processes;

Process* process_create(char* name, ProcessState state, VirtAddr ip, bool is_user, Process* parent)
{
	spin_acquire_force(&process_lock);

	Process* proc = kzalloc(sizeof(Process));
	strncpy(proc->name, name, sizeof(proc->name));

	proc->state = state;
	proc->id = pid_counter++;

	process_setup(proc, is_user);

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

	scheduler_add_process(&process_list, proc);
	thread_create(proc, ip, is_user);

	spin_free(&process_lock);
	return proc;
}

bool process_create_elf(char* name, ProcessState state, Process* parent, const char* path)
{
	spin_acquire_force(&process_lock);

	Process* proc = kzalloc(sizeof(Process));
	strncpy(proc->name, name, sizeof(proc->name));

	proc->state = state;
	proc->id = pid_counter++;

	process_setup(proc, true);

	proc->working_dir = vfs_get_root();
	proc->stack_top = CONFIG_user_stack_addr;

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
		proc_log("Unable to read \"%s\": %s\n", path, strerror(thread_errno));
		spin_free(&process_lock);
		return false;
	}

	// Load the executable into the new page map.
	ElfInfo info = {0};
	if (elf_load(proc->page_map, node->handle, 0, &info) == false)
	{
		proc_log("Unable to load \"%s\": %s\n", path, strerror(thread_errno));
		return false;
	}

	// If an interpreter was requested, load it at the configured base address.
	if (info.ld_path != NULL)
	{
		VfsNode* interp = vfs_get_node(vfs_get_root(), info.ld_path, true);
		if (interp == NULL)
		{
			proc_log("Unable to load interpreter \"%s\" for \"%s\": %s\n", info.ld_path, path, strerror(thread_errno));
			return false;
		}

		ElfInfo interp_info = {0};
		if (elf_load(proc->page_map, interp->handle, CONFIG_user_interp_base, &interp_info) == false)
		{
			proc_log("Unable to load interpreter \"%s\" for \"%s\": %s\n", info.ld_path, path, strerror(thread_errno));
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

	scheduler_add_process(&process_list, proc);
	thread_create(proc, info.entry_point, true);

	spin_free(&process_lock);
	return proc;
}

bool process_execve(const char* path, char** argv, char** envp)
{
	spin_acquire_force(&process_lock);
	scheduler_pause();

	// Use the current thread's structures.
	Thread* thread = arch_current_cpu()->thread;
	Process* proc = thread->parent;

	// Open the file and ensure it's there.
	VfsNode* node = vfs_get_node(vfs_get_root(), path, true);
	if (node == NULL)
	{
		proc_log("Unable to read \"%s\": %s\n", path, strerror(thread_errno));
		spin_free(&process_lock);
		return false;
	}

	process_setup(proc, true);

	// Create a new page map for the process.
	PageMap* map = vm_page_map_new();

	// Load the executable into the new page map.
	ElfInfo info = {0};
	if (elf_load(map, node->handle, 0, &info) == false)
	{
		proc_log("Unable to load \"%s\": %s\n", path, strerror(thread_errno));
		vm_page_map_destroy(map);
		spin_free(&process_lock);
		return false;
	}

	// If an interpreter was requested, load it at the configured base address.
	if (info.ld_path != NULL)
	{
		VfsNode* interp = vfs_get_node(vfs_get_root(), info.ld_path, true);
		if (interp == NULL)
		{
			proc_log("Unable to load interpreter \"%s\" for \"%s\": %s\n", info.ld_path, path, strerror(thread_errno));
			vm_page_map_destroy(map);
			spin_free(&process_lock);
			return false;
		}

		ElfInfo interp_info = {0};
		if (elf_load(map, interp->handle, CONFIG_user_interp_base, &interp_info) == false)
		{
			proc_log("Unable to load interpreter \"%s\" for \"%s\": %s\n", info.ld_path, path, strerror(thread_errno));
			vm_page_map_destroy(map);
			spin_free(&process_lock);
			return false;
		}
	}

	// If loading was successful, set the new map.
	thread->parent->page_map = map;

	// Set CWD.
	thread->parent->working_dir = node->parent;

	proc->map_base = CONFIG_vm_map_base;
	proc->stack_top = CONFIG_user_stack_addr;
	proc->state = ProcessState_Ready;

	// Map the process stack. Subtract size from the start since stack grows down.
	vm_map(map, CONFIG_user_stack_addr - CONFIG_user_stack_size, CONFIG_user_stack_size,
		   PROT_READ | PROT_WRITE | PROT_EXEC, MAP_FIXED, NULL, 0);

	arch_current_cpu()->user_stack = CONFIG_user_stack_addr;

	thread_execve(proc, thread, info.entry_point, argv, envp);

	vm_set_page_map(map);
	spin_free(&process_lock);

	// Run the scheduler.
	scheduler_invoke();

	return false;
}

usize process_fork(Process* proc, Thread* thread)
{
	spin_acquire_force(&process_lock);

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

	scheduler_add_process(&process_list, fork);
	thread_fork(fork, thread);

	fork->state = ProcessState_Ready;
	spin_free(&process_lock);

	return fork->id;
}

void process_kill(Process* proc, bool is_crash)
{
	scheduler_pause();
	if (proc->id <= 1)
		kmesg("[WARNING]\tKilling init or kernel process!\n");

	// If the process being killed is the currently running process.
	bool is_suicide = false;
	if (arch_current_cpu()->thread->parent == proc)
		is_suicide = true;

	// Hand the threads over to the hangman.
	spin_lock(&process_lock, {
		for (usize i = 0; i < proc->threads.length; i++)
		{
			scheduler_remove_thread(&thread_list, proc->threads.items[i]);
			scheduler_add_thread(&hanging_thread_list, proc->threads.items[i]);
		}
	});

	// Remove the process from its parent.
	if (proc->parent != NULL)
	{
		usize idx;
		if (list_find(&proc->parent->children, idx, proc))
			list_pop(&proc->parent->children, idx);
	}

	Process* init = process_list->next;

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

	spin_lock(&process_lock, {
		scheduler_remove_process(&process_list, proc);
		scheduler_add_process(&hanging_process_list, proc);
	});

	if (is_suicide)
		arch_current_cpu()->thread = NULL;

	scheduler_invoke();
}

FileDescriptor* process_fd_to_ptr(Process* process, usize fd)
{
	kassert(process != NULL, "No process specified! This is a kernel bug.");

	if (fd >= OPEN_MAX)
	{
		thread_errno = EBADF;
		return NULL;
	}

	FileDescriptor* file_desc = NULL;
	spin_lock(&process->fd_lock, {
		file_desc = process->file_descs[fd];
		if (file_desc == NULL)
		{
			thread_errno = EBADF;
			break;
		}
	});
	return file_desc;
}
