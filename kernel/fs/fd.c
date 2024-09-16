// File descriptor utils

#include <menix/abi.h>
#include <menix/arch.h>
#include <menix/fs/fd.h>
#include <menix/thread/process.h>
#include <menix/thread/spin.h>

#include <errno.h>

FileDescriptor* fd_from_num(Process* proc, int fd)
{
	FileDescriptor* result = NULL;
	if (proc == NULL)
		proc = arch_current_cpu()->thread->parent;

	spin_acquire_force(&proc->fd_lock);

	// Check if fd is inside bounds.
	if (fd < 0 || fd >= OPEN_MAX)
	{
		thread_errno = EBADF;
		goto leave;
	}

	result = proc->file_descs[fd];
	// If the fd number doesn't correspond to a file descriptor, set errno.
	if (result == NULL)
	{
		thread_errno = EBADF;
		goto leave;
	}

	result->num_refs++;

leave:
	spin_free(&proc->fd_lock);
	return result;
}
