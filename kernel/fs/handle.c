// Handle for managing input/output streams.

#include <menix/fs/fd.h>
#include <menix/fs/handle.h>
#include <menix/memory/alloc.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

#include <uapi/errno.h>

static isize handle_default_read(Handle* self, FileDescriptor* fd, void* buf, usize amount, off_t offset)
{
	thread_set_errno(ENOSYS);
	return -1;
}

static isize handle_default_write(Handle* self, FileDescriptor* fd, const void* buf, usize amount, off_t offset)
{
	thread_set_errno(ENOSYS);
	return -1;
}

static isize handle_default_ioctl(Handle* self, FileDescriptor* fd, u32 request, void* argument)
{
	switch (request)
	{
		case TCGETS:
		case TCSETS:
		case TIOCSCTTY:
		case TIOCGWINSZ: thread_set_errno(ENOTTY); return -1;
	}

	thread_set_errno(EINVAL);
	return -1;
}

void* handle_new(usize size)
{
	kassert(size >= sizeof(Handle), "Can't allocate a handle with less than %zu bytes, but only got %zu!",
			sizeof(Handle), size);
	Handle* result = kzalloc(size);

	result->read = handle_default_read;
	result->write = handle_default_write;
	result->ioctl = handle_default_ioctl;

	return result;
}

static usize device_counter = 1;
static SpinLock device_counter_lock = {0};

usize handle_new_device()
{
	spin_lock(&device_counter_lock);
	usize dev = device_counter++;
	spin_unlock(&device_counter_lock);
	return dev;
}
