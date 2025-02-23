#include <menix/fs/fd.h>
#include <menix/fs/handle.h>
#include <menix/fs/vfs.h>
#include <menix/memory/vm.h>
#include <menix/syscall/syscall.h>
#include <menix/system/arch.h>
#include <menix/system/sch/thread.h>

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
