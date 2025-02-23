#include <menix/common.h>
#include <menix/fs/fd.h>
#include <menix/fs/vfs.h>
#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>

#include <uapi/errno.h>

// Writes data from a buffer to a file descriptor.
// `fd`: The file descriptor to write to.
// `buf`: The data to write.
// `size`: The amount of data to write.
SYSCALL_IMPL(write, int fd, VirtAddr buf, usize size)
{
	if (size == 0 || buf == 0)
		return SYSCALL_ERR(EINVAL);

	Process* process = arch_current_cpu()->thread->parent;

	FileDescriptor* file_desc = fd_get(process, fd);
	if (file_desc == NULL)
		return SYSCALL_ERR(EBADF);

	VfsNode* const node = file_desc->node;
	if (node == NULL)
		return SYSCALL_ERR(ENOENT);
	Handle* const handle = node->handle;
	if (handle->read == NULL)
		return SYSCALL_ERR(ENOSYS);

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
SYSCALL_IMPL(read, int fd, VirtAddr buf, usize size)
{
	if (size == 0 || buf == 0)
		return SYSCALL_ERR(EINVAL);

	Process* process = arch_current_cpu()->thread->parent;

	FileDescriptor* file_desc = fd_get(process, fd);
	if (file_desc == NULL)
		return SYSCALL_ERR(EBADF);

	Handle* const handle = file_desc->node->handle;
	// Check if we can read.
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
	if (path == 0)
		return SYSCALL_ERR(EINVAL);

	Process* process = arch_current_cpu()->thread->parent;

	// Get parent descriptor.
	VfsNode* parent = NULL;
	if (fd == AT_FDCWD)
		parent = process->working_dir;
	else
	{
		FileDescriptor* file_desc = fd_get(process, fd);
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

	FileDescriptor* new_fd = fd_open(process, node);

	// We can't open any more files.
	if (new_fd == NULL)
		return SYSCALL_ERR(ENFILE);

	return SYSCALL_OK(new_fd->fd_num);
}

SYSCALL_IMPL(stat, int fd, VirtAddr path, int flags, VirtAddr buf)
{
	if (buf == 0)
		return SYSCALL_ERR(EINVAL);

	Process* process = arch_current_cpu()->thread->parent;
	VfsNode* node;

	// If we want to stat a file descriptor.
	if (fd != -1)
	{
		FileDescriptor* file_desc = fd_get(process, fd);
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

	if (fd_close(process, fd) == false)
		return SYSCALL_ERR(EBADF);

	return SYSCALL_OK(0);
}

SYSCALL_IMPL(ioctl, int fd, usize request, usize argument)
{
	// TODO
	return SYSCALL_ERR(ENOSYS);
}

SYSCALL_IMPL(seek, int fd, isize offset, int whence)
{
	Process* process = arch_current_cpu()->thread->parent;

	FileDescriptor* file_desc = fd_get(process, fd);
	if (file_desc == NULL)
		return SYSCALL_ERR(EBADF);

	const usize size = file_desc->node->handle->stat.st_size;

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
			file_desc->offset = size + offset;
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

SYSCALL_IMPL(fchdir, int fd)
{
	Process* process = arch_current_cpu()->thread->parent;

	FileDescriptor* file_desc = fd_get(process, fd);
	if (file_desc == NULL)
		return SYSCALL_ERR(EBADFD);

	if (!S_ISDIR(file_desc->node->handle->stat.st_mode))
		return SYSCALL_ERR(ENOTDIR);

	process->working_dir = file_desc->node;

	return SYSCALL_OK(0);
}

SYSCALL_IMPL(isatty, int fd)
{
	if (fd >= 0 && fd < 3)
		return SYSCALL_OK(0);
	return SYSCALL_OK(1);
}

SYSCALL_IMPL(faccessat, int dirfd, VirtAddr path, int mode, int flags)
{
	char* buf = kzalloc(PATH_MAX);
	vm_user_read(arch_current_cpu()->thread->parent, buf, path, PATH_MAX);
	// TODO
	kfree(buf);
	return SYSCALL_OK(0);
}

SYSCALL_IMPL(mkdirat, int fd, VirtAddr path, mode_t mode)
{
	if (path == 0)
		return SYSCALL_ERR(ENOENT);

	Process* proc = arch_current_cpu()->thread->parent;

	// Try to get the relative directory.
	VfsNode* at = NULL;
	if (fd == AT_FDCWD)
		at = proc->working_dir;
	else
	{
		FileDescriptor* file = fd_get(proc, fd);
		if (!file)
			return SYSCALL_ERR(EBADF);
		at = file->node;
	}

	// Read the file path from user mode.
	char* buf = kzalloc(PATH_MAX);
	vm_user_read(proc, buf, path, PATH_MAX);

	VfsNode* result_node = vfs_node_add(at, buf, mode);
	FileDescriptor* result_file = fd_open(proc, result_node);

	kfree(buf);
	return SYSCALL_OK(result_file->fd_num);
}

SYSCALL_STUB(chmodat)
SYSCALL_STUB(chownat)
SYSCALL_STUB(chroot)
SYSCALL_STUB(unmount)
SYSCALL_STUB(mount)
SYSCALL_STUB(unlinkat)
SYSCALL_STUB(readlinkat)
SYSCALL_STUB(linkat)
SYSCALL_STUB(sync)
SYSCALL_STUB(fcntl)
SYSCALL_STUB(readdir)
SYSCALL_STUB(umask)
SYSCALL_STUB(poll)
SYSCALL_STUB(rename)
