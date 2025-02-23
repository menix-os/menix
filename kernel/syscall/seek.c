#include <menix/fs/fd.h>
#include <menix/syscall/syscall.h>
#include <menix/system/sch/process.h>

#include <uapi/errno.h>

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
