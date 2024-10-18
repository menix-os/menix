#include <menix/abi/errno.h>
#include <menix/common.h>
#include <menix/syscall/syscall.h>
#include <menix/thread/process.h>

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
		return -1;
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
		vm_user_access({ node = vfs_get_node(parent, path, true); });
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

// Opens a connection between a file and a file descriptor. Returns a new file descriptor.
// `path`: The path to the file to be opened.
// `oflag`: Flags for opening the file.
// `mode`:
SYSCALL_IMPL(open, const char* path, int oflag, mode_t mode)
{
	return syscall_openat(AT_FDCWD, path, oflag, mode);
}
