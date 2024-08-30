// tmpfs file system

#include <menix/common.h>
#include <menix/fs/fs.h>
#include <menix/fs/vfs.h>
#include <menix/memory/alloc.h>
#include <menix/module.h>

static FileSystem tmpfs;

typedef struct
{
	Handle handle;		 // Underlying handle.
	void* buffer;		 // Start of the data buffer.
	usize buffer_cap;	 // Size of the data buffer.
} TmpHandle;

static dev_t device_id = 0;
static ino_t inode_counter = 0;

static TmpHandle* tmpfs_handle_new(FileSystem* fs, mode_t mode)
{
	TmpHandle* result = handle_new(sizeof(TmpHandle));

	result->handle.stat.st_size = 0;
	result->handle.stat.st_blocks = 0;
	result->handle.stat.st_blksize = 512;
	result->handle.stat.st_dev = device_id++;
	result->handle.stat.st_ino = inode_counter++;
	result->handle.stat.st_mode = mode;
	result->handle.stat.st_nlink = 1;
	return result;
}

static VfsNode* tmpfs_hard_link(FileSystem* self, VfsNode* parent, const char* name, VfsNode* target)
{
	// TODO
	return NULL;
}

static VfsNode* tmpfs_sym_link(FileSystem* self, VfsNode* parent, const char* name, const char* target)
{
	// TODO
	return NULL;
}

static VfsNode* tmpfs_create(FileSystem* self, VfsNode* parent, const char* name, mode_t mode)
{
	VfsNode* result = NULL;
	TmpHandle* handle = NULL;

	// Create a new node.
	result = vfs_node_new(&tmpfs, parent, name, S_ISDIR(mode));
	if (result == NULL)
		goto fail;

	handle = tmpfs_handle_new(&tmpfs, mode);
	if (handle == NULL)
		goto fail;

	result->handle = (Handle*)handle;
	return result;

fail:
	if (result != NULL)
		kfree(result);

	if (handle != NULL)
		kfree(handle);

	return NULL;
}

static VfsNode* tmpfs_mount(VfsNode* mount_point, const char* name, VfsNode* source)
{
	return tmpfs.create(&tmpfs, mount_point, name, 0644 | S_IFDIR);
}

static FileSystem tmpfs = {
	.name = "tmpfs",
	.mount = tmpfs_mount,
	.populate = NULL,	 // tmpfs has no persistent data storage, it can't populate anything.
	.create = tmpfs_create,
	.hard_link = tmpfs_hard_link,
	.sym_link = tmpfs_sym_link,
};

i32 tmpfs_init()
{
	return vfs_fs_register(&tmpfs);
}
