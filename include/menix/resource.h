// Resource management. Used for things like device files.

#pragma once
#include <menix/common.h>
#include <menix/thread/spin.h>

typedef struct FileDescriptor
{
	usize refs_num;				  // Amount of references to this descriptor.
	usize offset;				  // Current offset into the file.
	SpinLock lock;				  // Access lock.
	struct Resource* resource;	  // Resource connected to this descriptor.
} FileDescriptor;

typedef struct Resource
{
	SpinLock lock;	  // Access lock.

	// Read `amount` bytes of `this` resource from `offset` into `output_buffer`.
	usize (*read)(struct Resource* this, FileDescriptor* fd, void* output_buffer, usize amount, usize offset);
	// Write `amount` bytes of `input_buffer` into `this` resource from `offset`.
	usize (*write)(struct Resource* this, FileDescriptor* fd, const void* input_buffer, usize amount, usize offset);
	// Map `this` into virtual memory.
	void* (*mmap)(struct Resource* this, usize fd_page, usize flags);
} Resource;
