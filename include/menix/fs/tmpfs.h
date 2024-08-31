// tmpfs file system

#pragma once
#include <menix/fs/fs.h>
#include <menix/fs/handle.h>

// Wrapper around a handle.
typedef struct
{
	Handle handle;		 // Underlying handle.
	void* buffer;		 // Start of the data buffer.
	usize buffer_cap;	 // Size of the data buffer.
} TmpHandle;

// Initializes the FS.
i32 tmpfs_init();
