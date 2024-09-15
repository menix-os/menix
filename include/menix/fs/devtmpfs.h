// devtmpfs file system

#pragma once
#include <menix/fs/handle.h>

// Initializes the FS.
i32 devtmpfs_init();

// Adds a device to the devtmpfs.
bool devtmpfs_add_device(Handle* device, const char* name);

// Adds /dev/zero /dev/null and /dev/full to the VFS.
void devtmpfs_register_default();
