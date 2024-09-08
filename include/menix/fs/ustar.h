// UStar file system

#pragma once

#include <menix/common.h>
#include <menix/fs/vfs.h>

i32 ustarfs_init(VfsNode* mount, void* data, usize size);
