// Resource management. Used for things like device files.

#pragma once
#include <menix/common.h>
#include <menix/thread/spin.h>

typedef struct Resource Resource;

typedef struct FileDescriptor
{
	usize num_refs;		   // Amount of references to this descriptor.
	usize offset;		   // Current offset into the file.
	SpinLock lock;		   // Access lock.
	Resource* resource;	   // Resource connected to this descriptor.
} FileDescriptor;

typedef struct Resource
{
	SpinLock lock;	  // Access lock.

	// Read `amount` bytes of `this` resource from `offset` into `output_buffer`.
	usize (*read)(Resource* this, FileDescriptor* fd, void* output_buffer, usize amount, usize offset);
	// Write `amount` bytes of `input_buffer` into `this` resource from `offset`.
	usize (*write)(Resource* this, FileDescriptor* fd, const void* input_buffer, usize amount, usize offset);
	// Map `this` into virtual memory.
	void* (*mmap)(Resource* this, usize fd_page, usize flags);
} Resource;
