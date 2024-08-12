// Virtual File System

#include <menix/fs/vfs.h>
#include <menix/log.h>
#include <menix/thread/spin.h>

static VfsNode* vfs_root;
static SpinLock vfs_lock = spin_new();

void vfs_init()
{
	// TODO
	// vfs_node_new(NULL, NULL, "", false);	// Create root.

	kmesg("Initialized virtual file system.\n");
}

VfsNode* vfs_get_root()
{
	return vfs_root;
}
