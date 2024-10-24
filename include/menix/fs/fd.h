// File descriptor structure

#pragma once
#include <menix/common.h>
#include <menix/util/spin.h>

typedef struct FileDescriptor
{
	struct Handle* handle;	  // Handle connected to this descriptor.
	usize num_refs;			  // Amount of references to this descriptor.
	usize offset;			  // Current offset into the file.
	struct VfsNode* node;	  // The node that this descriptor is pointing to.
	SpinLock lock;			  // Access lock.
} FileDescriptor;

typedef struct Process Process;

// Looks up a file descriptor number in a process and returns the corresponding data.
FileDescriptor* fd_from_num(Process* proc, int fd);
