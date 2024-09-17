// Handle for managing input/output streams.

#pragma once
#include <menix/fs/fd.h>
#include <menix/system/abi.h>
#include <menix/thread/spin.h>

typedef struct Handle
{
	SpinLock lock;		 // Access lock.
	struct stat stat;	 // POSIX handle status.

	// Read `amount` bytes of `fd` from `offset` into `output_buffer`. Returns actual bytes read.
	isize (*read)(struct Handle* self, FileDescriptor* fd, void* output_buffer, usize amount, off_t offset);
	// Write `amount` bytes of `input_buffer` into `fd` from `offset`. Returns actual bytes written.
	isize (*write)(struct Handle* self, FileDescriptor* fd, const void* input_buffer, usize amount, off_t offset);
	// Executes a `request` with an `argument` with an implementation defined function in `fd`.
	isize (*ioctl)(struct Handle* self, FileDescriptor* fd, u32 request, void* argument);
} Handle;

// Allocates and initializes a new handle.
// The function pointers are set to a default implementation instead of null.
// `size`: The size of the handle. Needs to be at least sizeof(Handle) bytes.
void* handle_new(usize size);

// Returns a new dev_t ID for use by a Handle structure.
usize handle_new_device();
