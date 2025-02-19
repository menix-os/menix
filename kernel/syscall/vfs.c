#include <menix/common.h>
#include <menix/fs/fd.h>
#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>

#include <uapi/errno.h>

// Writes data from a buffer to a file descriptor.
// `fd`: The file descriptor to write to.
// `buf`: The data to write.
// `size`: The amount of data to write.
SYSCALL_IMPL(write, u32 fd, VirtAddr buf, usize size)
{
	if (size == 0 || buf == 0)
		return SYSCALL_ERR(EINVAL);

	Process* process = arch_current_cpu()->thread->parent;

	FileDescriptor* file_desc = proc_fd_to_ptr(process, fd);
	if (file_desc == NULL)
		return SYSCALL_ERR(EBADF);

	Handle* const handle = file_desc->handle;
	if (handle == NULL)
		return SYSCALL_ERR(ENOMEM);
	if (handle->write == NULL)
		return SYSCALL_ERR(ENOMEM);

	// Copy data from user.
	void* kernel_buf = kmalloc(size);
	vm_user_read(process, kernel_buf, buf, size);

	// Write to the handle.
	isize result = handle->write(handle, file_desc, kernel_buf, size, file_desc->offset);
	file_desc->offset += result;

	kfree(kernel_buf);

	return SYSCALL_OK(result);
}

// Reads from a file descriptor to a buffer.
// `fd`: The file descriptor to read from.
// `buf`: A buffer to write to.
// `size`: The amount of data to read.
SYSCALL_IMPL(read, u32 fd, VirtAddr buf, usize size)
{
	if (size == 0 || buf == 0)
		return SYSCALL_ERR(EINVAL);

	Process* process = arch_current_cpu()->thread->parent;

	FileDescriptor* file_desc = proc_fd_to_ptr(process, fd);
	if (file_desc == NULL)
		return SYSCALL_ERR(EBADF);

	Handle* const handle = file_desc->handle;
	if (handle == NULL)
		return SYSCALL_ERR(ENOENT);
	if (handle->read == NULL)
		return SYSCALL_ERR(ENOSYS);

	void* kernel_buf = kmalloc(size);

	// Read from the handle.
	isize result = handle->read(handle, file_desc, kernel_buf, size, file_desc->offset);
	file_desc->offset += result;

	// Copy data to user.
	vm_user_write(process, buf, kernel_buf, size);
	kfree(kernel_buf);

	return SYSCALL_OK(result);
}

// Opens a connection between a file and a file descriptor. Returns a file descriptor or -1 if it failed.
// `fd`: The file descriptor root.
// `path`: The path to the file to be opened, relative to fd.
// `buf`: A buffer to write to.
// `size`: The amount of data to read.
SYSCALL_IMPL(openat, int fd, VirtAddr path, int oflag, mode_t mode)
{
	Process* process = arch_current_cpu()->thread->parent;

	if (path == 0)
		return SYSCALL_ERR(EINVAL);

	// Get parent descriptor.
	VfsNode* parent = NULL;
	if (fd == AT_FDCWD)
		parent = process->working_dir;
	else
	{
		FileDescriptor* file_desc = proc_fd_to_ptr(process, fd);
		if (file_desc == NULL)
			return SYSCALL_ERR(EBADF);

		parent = file_desc->node;
	}

	if (parent == NULL)
		return SYSCALL_ERR(ENOENT);

	// If there is a parent, find the requested node relative to it.
	VfsNode* node;
	char* kernel_path = kmalloc(PATH_MAX);
	vm_user_read(process, kernel_path, path, PATH_MAX);

	node = vfs_get_node(parent, kernel_path, true);
	kfree(kernel_path);
	if (node == NULL)
		return SYSCALL_ERR(ENOENT);

	// The node was found, allocate a new file descriptor.
	int last_fd = -1;
	// Find a free fd number.
	for (int i = 0; i < OPEN_MAX; i++)
	{
		if (process->file_descs[i] == NULL)
		{
			last_fd = i;
			break;
		}
	}

	// We can't open any more files.
	if (last_fd == -1)
		return SYSCALL_ERR(ENFILE);

	FileDescriptor* new_fd = kzalloc(sizeof(FileDescriptor));
	new_fd->num_refs++;
	new_fd->handle = node->handle;
	process->file_descs[last_fd] = new_fd;

	return SYSCALL_OK(last_fd);
}

SYSCALL_IMPL(stat, int fd, VirtAddr path, VirtAddr buf)
{
	Process* process = arch_current_cpu()->thread->parent;
	VfsNode* node;

	// If we want to stat a file descriptor.
	if (fd != -1)
	{
		FileDescriptor* file_desc = proc_fd_to_ptr(process, fd);
		if (file_desc == NULL)
			return SYSCALL_ERR(EBADF);

		node = file_desc->node;
	}
	else
	{
		if (path == 0)
			return SYSCALL_ERR(EINVAL);

		char* kernel_path = kmalloc(PATH_MAX);
		vm_user_read(process, kernel_path, path, PATH_MAX);

		node = vfs_get_node(process->working_dir, kernel_path, true);
		kfree(kernel_path);
	}

	if (node == NULL)
		return SYSCALL_ERR(ENOENT);

	vm_user_write(process, buf, &node->handle->stat, sizeof(struct stat));

	return SYSCALL_OK(0);
}

// Closes a file descriptor.
// `fd`: The file descriptor to close.
SYSCALL_IMPL(close, int fd)
{
	Process* process = arch_current_cpu()->thread->parent;

	FileDescriptor* file_desc = proc_fd_to_ptr(process, fd);
	if (file_desc == NULL)
		return SYSCALL_ERR(EBADF);

	// If the file descriptor exists, lose the reference.
	process->file_descs[fd] = NULL;

	// Decrement the counter.
	spin_lock_scope(&file_desc->lock, { file_desc->num_refs -= 1; });

	return SYSCALL_OK(0);
}

SYSCALL_STUB(ioctl, u32 fd, u32 request, void* argument)

SYSCALL_IMPL(seek, int fd, isize offset, int whence)
{
	Process* process = arch_current_cpu()->thread->parent;

	FileDescriptor* file_desc = proc_fd_to_ptr(process, fd);
	if (file_desc == NULL)
		return SYSCALL_ERR(EBADF);

	// TODO: Check offset bounds.
	switch (whence)
	{
		case SEEK_SET:
		{
			file_desc->offset = offset;
			break;
		}
		case SEEK_CUR:
		{
			file_desc->offset += offset;
			break;
		}
		case SEEK_END:
		{
			file_desc->offset = file_desc->handle->stat.st_size + offset;
			break;
		}
		default: return SYSCALL_ERR(EINVAL);
	}
	return SYSCALL_OK(file_desc->offset);
}

SYSCALL_IMPL(chdir, VirtAddr path)
{
	Process* process = arch_current_cpu()->thread->parent;
	if (path == 0)
		return SYSCALL_ERR(EINVAL);

	char* kernel_path = kmalloc(PATH_MAX);
	vm_user_read(process, kernel_path, path, PATH_MAX);

	VfsNode* new_cwd = vfs_get_node(process->working_dir, kernel_path, true);
	kfree(kernel_path);
	if (new_cwd == NULL)
		return SYSCALL_ERR(ENOENT);

	if (!S_ISDIR(new_cwd->handle->stat.st_mode))
		return SYSCALL_ERR(ENOTDIR);

	process->working_dir = new_cwd;

	return SYSCALL_OK(0);
}

SYSCALL_IMPL(fchdir, usize fd)
{
	Process* process = arch_current_cpu()->thread->parent;

	FileDescriptor* file_desc = proc_fd_to_ptr(process, fd);
	if (file_desc == NULL)
		return SYSCALL_ERR(EBADFD);

	if (!S_ISDIR(file_desc->handle->stat.st_mode))
		return SYSCALL_ERR(ENOTDIR);

	process->working_dir = file_desc->node;

	return SYSCALL_OK(0);
}

SYSCALL_STUB(access)
SYSCALL_STUB(faccessat)
SYSCALL_STUB(chmodat)
SYSCALL_STUB(chownat)
SYSCALL_STUB(chroot)
SYSCALL_STUB(unmount)
SYSCALL_STUB(mount)
SYSCALL_STUB(unlinkat)
SYSCALL_STUB(readlinkat)
SYSCALL_STUB(linkat)
SYSCALL_STUB(mkdirat)
SYSCALL_STUB(sync)
SYSCALL_STUB(isatty)
SYSCALL_STUB(fcntl)
SYSCALL_STUB(readdir)
SYSCALL_STUB(umask)
SYSCALL_STUB(poll)
SYSCALL_STUB(rename)
