// Handle for managing input/output streams.

#include <menix/arch.h>
#include <menix/fs/fd.h>
#include <menix/fs/handle.h>
#include <menix/memory/alloc.h>
#include <menix/thread/proc.h>

#include <errno.h>

static isize handle_default_read(Handle* self, FileDescriptor* fd, void* buf, usize amount, off_t offset)
{
	usize* errno = &arch_current_cpu()->thread->errno;
	*errno = ENOSYS;
	return -1;
}

static isize handle_default_write(Handle* self, FileDescriptor* fd, const void* buf, usize amount, off_t offset)
{
	usize* errno = &arch_current_cpu()->thread->errno;
	*errno = ENOSYS;
	return -1;
}

void* handle_new(usize size)
{
	kassert(size >= sizeof(Handle), "Can't allocate a handle with less than %zu bytes, but only got %zu!",
			sizeof(Handle), size);
	Handle* result = kzalloc(size);

	result->read = handle_default_read;
	result->write = handle_default_write;

	return result;
}
