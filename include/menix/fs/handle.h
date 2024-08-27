// Handle for managing input/output streams.

#pragma once
#include <menix/fs/descriptor.h>
#include <menix/thread/spin.h>

typedef struct DataHandle
{
	SpinLock lock;	  // Access lock.

	// Read `amount` bytes of `self` from `offset` into `output_buffer`. Returns actual bytes read.
	isize (*read)(struct DataHandle* self, FileDescriptor* fd, void* output_buffer, usize amount, usize offset);
	// Write `amount` bytes of `input_buffer` into `self` from `offset`. Returns actual bytes written.
	isize (*write)(struct DataHandle* self, FileDescriptor* fd, const void* input_buffer, usize amount, usize offset);
	// Write `amount` bytes of `input_buffer` into `self` from `offset`. Returns actual bytes written.
	isize (*ioctl)(struct DataHandle* self, FileDescriptor* fd, usize request, usize argument);
} DataHandle;
