#include <menix/fs/fd.h>
#include <menix/fs/handle.h>
#include <menix/fs/vfs.h>
#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/thread.h>

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
