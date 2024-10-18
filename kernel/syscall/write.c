#include <menix/common.h>
#include <menix/syscall/syscall.h>
#include <menix/thread/process.h>

// Writes data from a buffer to a file descriptor.
// `fd`: The file descriptor to write to.
// `buf`: The data to write.
// `size`: The amount of data to write.
SYSCALL_IMPL(write, u32 fd, void* buf, usize size)
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
	if (handle->write == NULL)
		return 0;

	// Write to the handle.
	isize result = 0;
	vm_user_access({ result = handle->write(handle, file_desc, buf, size, file_desc->offset); });

	return result;
}
