#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>

// Writes data from a buffer to a file descriptor.
// `fd`: The file descriptor to write to.
// `buf`: The data to write.
// `size`: The amount of data to write.
SYSCALL_IMPL(write, u32 fd, void* buf, usize size)
{
	if (size == 0 || buf == NULL)
		return 0;

	Process* process = arch_current_cpu()->thread->parent;
	if (!vm_is_mapped(process->page_map, (VirtAddr)buf, VMProt_Write))
		return -ENOMEM;

	FileDescriptor* file_desc = process_fd_to_ptr(process, fd);
	if (file_desc == NULL)
		return 0;

	Handle* const handle = file_desc->handle;
	if (handle == NULL)
		return 0;
	if (handle->write == NULL)
		return 0;

	// Write to the handle.
	isize result = handle->write(handle, file_desc, buf, size, file_desc->offset);

	return result;
}

// Opens a connection between a file and a file descriptor. Returns a file descriptor or -1 if it failed.
// `fd`: The file descriptor root.
// `path`: The path to the file to be opened, relative to fd.
// `buf`: A buffer to write to.
// `size`: The amount of data to read.
SYSCALL_IMPL(openat, int fd, const char* path, int oflag, mode_t mode)
{
	Process* process = arch_current_cpu()->thread->parent;

	if (path == NULL)
	{
		thread_errno = ENOENT;
		return -ENOENT;
	}

	// Get parent descriptor.
	VfsNode* parent = NULL;
	if (fd == AT_FDCWD)
	{
		parent = process->working_dir;
	}
	else
	{
		FileDescriptor* file_desc = process_fd_to_ptr(process, fd);
		if (file_desc == NULL)
		{
			return -1;
		}
		parent = file_desc->node;
	}

	// If there is a parent, find the requested node relative to it.
	if (parent != NULL)
	{
		VfsNode* node;
		node = vfs_get_node(parent, path, true);
		if (node == NULL)
			return -1;

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
		{
			thread_errno = ENFILE;
			return -1;
		}

		FileDescriptor* new_fd = kzalloc(sizeof(FileDescriptor));
		new_fd->num_refs++;
		new_fd->handle = node->handle;
		process->file_descs[last_fd] = new_fd;

		return last_fd;
	}

	return -1;
}

// Reads from a file descriptor to a buffer.
// `fd`: The file descriptor to read from.
// `buf`: A buffer to write to.
// `size`: The amount of data to read.
SYSCALL_IMPL(read, u32 fd, void* buf, usize size)
{
	if (size == 0 || buf == NULL)
		return 0;

	Process* process = arch_current_cpu()->thread->parent;

	FileDescriptor* file_desc = process_fd_to_ptr(process, fd);
	if (file_desc == NULL)
		return 0;

	Handle* const handle = file_desc->handle;
	if (handle == NULL)
		return 0;
	if (handle->read == NULL)
		return 0;

	// Read from the handle.
	isize result = handle->read(handle, file_desc, buf, size, file_desc->offset);

	return result;
}

SYSCALL_IMPL(stat)
{
	// TODO
	return 0;
}

// Opens a connection between a file and a file descriptor. Returns a new file descriptor.
// `path`: The path to the file to be opened.
// `oflag`: Flags for opening the file.
// `mode`:
SYSCALL_IMPL(open, const char* path, int oflag, mode_t mode)
{
	return syscall_openat(AT_FDCWD, path, oflag, mode);
}

// Closes a file descriptor.
// `fd`: The file descriptor to close.
SYSCALL_IMPL(close, int fd)
{
	// TODO
	return 0;
}

SYSCALL_IMPL(ioctl, u32 fd, u32 request, void* argument)
{
	// TODO
	return 0;
}

SYSCALL_STUB(access)
SYSCALL_STUB(seek)
SYSCALL_STUB(chmod)
SYSCALL_STUB(chown)
SYSCALL_STUB(unmount)
SYSCALL_STUB(mount)
SYSCALL_STUB(chdir)
SYSCALL_STUB(unlink)
SYSCALL_STUB(symlink)
SYSCALL_STUB(readlink)
SYSCALL_STUB(link)
SYSCALL_STUB(rmdir)
SYSCALL_STUB(sync)
