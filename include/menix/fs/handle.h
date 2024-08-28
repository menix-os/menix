// Handle for managing input/output streams.

#pragma once
#include <menix/fs/descriptor.h>
#include <menix/thread/spin.h>

typedef struct Handle
{
	SpinLock lock;	  // Access lock.

	// Read `amount` bytes of `fd` from `offset` into `output_buffer`. Returns actual bytes read.
	isize (*read)(struct Handle* self, FileDescriptor* fd, void* output_buffer, usize amount, usize offset);
	// Write `amount` bytes of `input_buffer` into `fd` from `offset`. Returns actual bytes written.
	isize (*write)(struct Handle* self, FileDescriptor* fd, const void* input_buffer, usize amount, usize offset);
	// Executes a `request` with an `argument` with an implementation defined function in `fd`.
	isize (*ioctl)(struct Handle* self, FileDescriptor* fd, u32 request, void* argument);
} Handle;
