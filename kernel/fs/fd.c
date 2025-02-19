// File descriptor utils

#include <menix/fs/fd.h>
#include <menix/system/abi.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/util/spin.h>

#include <uapi/errno.h>

FileDescriptor* fd_from_num(Process* proc, int fd)
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

	result->num_refs++;

leave:
	spin_unlock(&proc->fd_lock);
	return result;
}
