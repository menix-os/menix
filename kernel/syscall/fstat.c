#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>
#include <menix/system/sch/scheduler.h>

SYSCALL_IMPL(fstat, int fd, VirtAddr path, int flags, VirtAddr buf)
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
		{
			return SYSCALL_ERR(EBADF);
		}

		node = file_desc->node;
	}
	else
	{
		if (path == 0)
		{
			return SYSCALL_ERR(EINVAL);
		}

		char* kernel_path = kmalloc(PATH_MAX);
		vm_user_read(process, kernel_path, path, PATH_MAX);

		node = vfs_get_node(process->working_dir, kernel_path, true);
		kfree(kernel_path);
	}

	if (node == NULL)
	{
		return SYSCALL_ERR(ENOENT);
	}

	vm_user_write(process, buf, &node->handle->stat, sizeof(struct stat));

	return SYSCALL_OK(0);
}
