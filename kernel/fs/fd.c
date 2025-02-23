// File descriptor utils

#include <menix/fs/fd.h>
#include <menix/system/abi.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

#include <uapi/errno.h>

FileDescriptor* fd_open(Process* process, VfsNode* node)
{
	// Find a free fd number.
	for (int i = 0; i < OPEN_MAX; i++)
	{
		if (process->file_descs[i] == NULL)
		{
			FileDescriptor* result = kzalloc(sizeof(FileDescriptor));
			result->node = node;
			result->fd_num = i;
			process->file_descs[result->fd_num] = result;
			return result;
		}
	}

	return NULL;
}

FileDescriptor* fd_get(Process* proc, int fd)
{
	FileDescriptor* result = NULL;
	if (proc == NULL)
		proc = arch_current_cpu()->thread->parent;

	spin_lock(&proc->fd_lock);

	// Check if fd is inside bounds.
	if (fd < 0 || fd >= OPEN_MAX)
	{
		thread_set_errno(EBADF);
		goto leave;
	}

	result = proc->file_descs[fd];

	// If the fd number doesn't correspond to a file descriptor, set errno.
	if (result == NULL)
	{
		thread_set_errno(EBADF);
		goto leave;
	}

leave:
	spin_unlock(&proc->fd_lock);
	return result;
}

bool fd_close(Process* proc, int fd)
{
	kassert(proc != NULL, "No process given!");

	if (fd < 0 || fd >= OPEN_MAX || proc->file_descs[fd] == NULL)
	{
		return false;
	}

	kfree(proc->file_descs[fd]);
	proc->file_descs[fd] = NULL;
	return true;
}
