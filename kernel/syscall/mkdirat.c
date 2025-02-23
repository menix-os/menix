#include <menix/syscall/syscall.h>
#include <menix/system/abi.h>
#include <menix/system/sch/process.h>

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
