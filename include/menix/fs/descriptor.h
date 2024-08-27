// File descriptor structure

#pragma once
#include <menix/common.h>
#include <menix/thread/spin.h>

typedef struct DataHandle DataHandle;

typedef struct FileDescriptor
{
	DataHandle* handle;	   // Handle connected to this descriptor.
	usize num_refs;		   // Amount of references to this descriptor.
	usize offset;		   // Current offset into the file.
	SpinLock lock;		   // Access lock.
} FileDescriptor;
