#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>

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
