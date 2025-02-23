#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>

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
