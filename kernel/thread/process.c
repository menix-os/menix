// Process creation

#include <menix/common.h>
#include <menix/fs/vfs.h>
#include <menix/memory/alloc.h>
#include <menix/memory/vm.h>
#include <menix/thread/elf.h>
#include <menix/thread/process.h>
#include <menix/thread/spin.h>

#include <errno.h>
#include <string.h>

static SpinLock lock = spin_new();
static usize pid_counter = 0;

void process_create(char* name, ProcessState state, usize ip, bool is_user, Process* parent)
{
	spin_acquire_force(&lock);

	Process* proc = kzalloc(sizeof(Process));
	strncpy(proc->name, name, sizeof(proc->name));

	spin_free(&lock);
}

bool process_execve(const char* path, char** argv, char** envp)
{
	spin_acquire_force(&lock);

	// Open the file and ensure it's there.
	VfsNode* node = vfs_get_node(vfs_get_root(), path, true);
	if (node == NULL)
	{
		proc_log("Unable to load file \"%s\": %s\n", path, strerror(thread_errno));
		return false;
	}

	// Create a new page map for the process.
	PageMap* map = vm_page_map_new();

	// Load the executable into the new page map.
	ElfInfo info = {0};
	if (elf_load(map, node->handle, 0, &info) == false)
	{
		proc_log("Unable to load file \"%s\"\n", path);
		return false;
	}

	// If an interpreter was requested, load it at the configured base address.
	if (info.ld_path != NULL)
	{
		VfsNode* interp = vfs_get_node(vfs_get_root(), info.ld_path, true);
		if (interp == NULL)
		{
			proc_log("Unable to load interpreter \"%s\" for \"%s\"\n", info.ld_path, path);
			return false;
		}
		ElfInfo interp_info;
		elf_load(map, interp->handle, CONFIG_user_interp_base, &interp_info);
	}

	// Allocate a new process structure.
	Thread* thread = arch_current_cpu()->thread;
	thread->parent = kzalloc(sizeof(Process));
	thread->parent->page_map = map;

	// TODO: Open stdout, stdin and stderr
	FileDescriptor* fd = kzalloc(sizeof(FileDescriptor));
	fd->handle = vfs_get_node(vfs_get_root(), "/dev/terminal0", true)->handle;
	thread->parent->file_descs[0] = fd;
	thread->parent->file_descs[1] = fd;
	thread->parent->file_descs[2] = fd;

	// Set CWD.
	thread->parent->working_dir = node->parent;

	// Map the process stack. Subtract size from the start since stack grows down.
	vm_map(map, CONFIG_user_stack_addr - CONFIG_user_stack_size, CONFIG_user_stack_size,
		   PROT_READ | PROT_WRITE | PROT_EXEC, MAP_FIXED, NULL, 0);

	arch_current_cpu()->user_stack = CONFIG_user_stack_addr;

	vm_set_page_map(map);
	asm_get_register(arch_current_cpu()->kernel_stack, rsp);
	spin_free(&lock);
	arch_return_to_user(info.entry_point);
	return false;
}

usize process_fork(Process* proc, Thread* thread)
{
	spin_acquire_force(&lock);

	Process* fork = kzalloc(sizeof(Process));
	fixed_strncpy(fork->name, proc->name);

	fork->id = pid_counter++;
	fork->parent = proc;
	fork->working_dir = proc->working_dir;

	fork->children = (ProcessList) {0};
	fork->threads = (ThreadList) {0};

	spin_free(&lock);
	return fork->id;
}

void process_kill(Process* proc)
{
	// TODO
	while (1)
		;
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
