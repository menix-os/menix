#include <menix/common.h>
#include <menix/syscall/syscall.h>
#include <menix/thread/process.h>

// Reads from a file descriptor to a buffer.
// `fd`: The file descriptor to read from.
// `buf`: A buffer to write to.
// `size`: The amount of data to read.
SYSCALL_IMPL(read, u32 fd, void* buf, usize size)
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
	if (handle->read == NULL)
		return 0;

	// Read from the handle.
	isize result = 0;
	vm_user_access({ result = handle->read(handle, file_desc, buf, size, file_desc->offset); });

	return result;
}
