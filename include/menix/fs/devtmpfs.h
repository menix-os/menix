// devtmpfs file system

#pragma once
#include <menix/fs/handle.h>

// Initializes the FS.
i32 devtmpfs_init();

// Adds a device to the devtmpfs.
bool devtmpfs_add_device(Handle* device, const char* name);
