// File descriptor structure

#pragma once
#include <menix/common.h>
#include <menix/util/spin.h>

typedef struct VfsNode VfsNode;

typedef struct FileDescriptor
{
	int fd_num;		  // The file descriptor ID.
	usize offset;	  // Current offset into the file.
	VfsNode* node;	  // The node that this descriptor is pointing to.
	SpinLock lock;	  // Access lock.
} FileDescriptor;

typedef struct Process Process;

// Creates a new file descriptor for an VFS node in `proc`.
FileDescriptor* fd_open(Process* proc, VfsNode* node);

// Opens a file descriptor for the process.
FileDescriptor* fd_get(Process* proc, int fd);

// Closes a file descriptor for the process.
bool fd_close(Process* proc, int fd);
