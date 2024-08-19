// Virtual File System

#include <menix/fs/vfs.h>
#include <menix/log.h>
#include <menix/memory/alloc.h>
#include <menix/thread/spin.h>

#include <string.h>

static SpinLock vfs_lock = spin_new();
static VfsNode* vfs_root;
static HashMap(VfsMountFn) fs_map;

void vfs_init()
{
	// Create root.
	vfs_node_new(NULL, NULL, "", false);

	// Allocate a hashmap for file systems.
	hashmap_init(fs_map, 64);

	kmesg("Initialized virtual file system.\n");
}

bool vfs_fs_register(VfsMountFn mount, const char* id)
{
	spin_acquire_force(&vfs_lock);
	hashmap_insert(&fs_map, id, strlen(id), mount);
	spin_free(&vfs_lock);

	return true;
}

VfsNode* vfs_get_root()
{
	return vfs_root;
}

VfsNode* vfs_node_new(FileSystem* fs, VfsNode* parent, const char* name, bool is_dir)
{
	VfsNode* node = kmalloc(sizeof(VfsNode));

	node->parent = parent;
	node->fs = fs;

	// If the new node is a directory, make room for potential children.
	if (is_dir)
		hashmap_init(node->children, 128);

	// Copy the name to the new node.
	const usize name_len = strlen(name);
	node->name = kmalloc(name_len + 1);
	memcpy(node->name, name, name_len);

	return node;
}

VfsNode* vfs_get_node(VfsNode* parent, const char* path, bool follow_links)
{
	spin_acquire_force(&vfs_lock);

	VfsNode* ret = NULL;
	// TODO

	spin_free(&vfs_lock);
	return ret;
}
