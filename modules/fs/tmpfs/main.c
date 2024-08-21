// tmpfs file system

#include <menix/common.h>
#include <menix/fs/fs.h>
#include <menix/fs/vfs.h>
#include <menix/memory/alloc.h>
#include <menix/module.h>

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

static VfsNode* tmpfs_create(FileSystem* self, VfsNode* parent, const char* name, ModeId mode)
{
	// TODO
	return NULL;
}

static VfsNode* tmpfs_mount(VfsNode* mount_point, const char* name, VfsNode* source)
{
	// TODO
	return NULL;
}

static FileSystem tmpfs = {
	.name = "tmpfs",
	.mount = tmpfs_mount,
	.populate = NULL,	 // tmpfs has no persistent data storage, it can't populate anything.
	.create = tmpfs_create,
	.hard_link = tmpfs_hard_link,
	.sym_link = tmpfs_sym_link,
};

MODULE_FN i32 tmpfs_init()
{
	return vfs_fs_register(&tmpfs);
}

MODULE = {
	.name = MODULE_NAME,
	.init = tmpfs_init,
	MODULE_META,
};
