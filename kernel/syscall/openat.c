#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>

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
