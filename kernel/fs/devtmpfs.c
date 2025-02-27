// Temporary file system for device files.

#include <menix/common.h>
#include <menix/fs/devtmpfs.h>
#include <menix/fs/fs.h>
#include <menix/fs/tmpfs.h>
#include <menix/fs/vfs.h>
#include <menix/memory/alloc.h>
#include <menix/system/arch.h>
#include <menix/system/module.h>
#include <menix/system/sch/process.h>
#include <menix/util/hash_map.h>
#include <menix/util/spin.h>

#include <uapi/errno.h>

#include <string.h>

static FileSystem devtmpfs;

static dev_t device_id = 0;
static ino_t inode_counter = 0;

static isize null_read(Handle* self, FileDescriptor* fd, void* buffer, usize amount, off_t offset)
{
	return 0;
}

static isize full_read(Handle* self, FileDescriptor* fd, void* buffer, usize amount, off_t offset)
{
	memset(buffer, 0, amount);
	return 0;
}

static isize zero_read(Handle* self, FileDescriptor* fd, void* buffer, usize amount, off_t offset)
{
	memset(buffer, 0, amount);
	return amount;
}

static isize null_write(Handle* self, FileDescriptor* fd, const void* buffer, usize amount, off_t offset)
{
	return amount;
}

static isize full_write(Handle* self, FileDescriptor* fd, const void* buffer, usize amount, off_t offset)
{
	return -1;
}

static isize zero_write(Handle* self, FileDescriptor* fd, const void* buffer, usize amount, off_t offset)
{
	return amount;
}

static isize devtmpfs_handle_read(struct Handle* self, FileDescriptor* fd, void* buffer, usize amount, off_t offset)
{
	spin_lock(&self->lock);

	TmpHandle* const handle = (TmpHandle*)self;
	isize total_read = amount;

	// Calculate the maximum amount of data one can actually read from the buffer.
	// If we're reading past the end of the file, subtract that from the amount read.
	if ((amount + offset) >= self->stat.st_size)
		total_read -= ((amount + offset) - self->stat.st_size);

	// Copy all data to the buffer.
	memcpy(buffer, handle->buffer, total_read);

	spin_unlock(&self->lock);
	return total_read;
}

static isize devtmpfs_handle_write(struct Handle* self, FileDescriptor* fd, const void* buffer, usize amount,
								   off_t offset)
{
	spin_lock(&self->lock);

	TmpHandle* const handle = (TmpHandle*)self;

	isize written = -1;

	if (offset + amount >= handle->buffer_cap)
	{
		usize new_capacity = handle->buffer_cap;
		while (offset + amount >= new_capacity)
			new_capacity *= 2;

		void* new_data = krealloc(handle->buffer, new_capacity);
		if (new_data == NULL)
		{
			thread_set_errno(ENOMEM);
			goto fail;
		}

		handle->buffer = new_data;
		handle->buffer_cap = new_capacity;
	}

	memcpy(handle->buffer + offset, buffer, amount);

	if ((amount + offset) >= self->stat.st_size)
	{
		self->stat.st_size = (off_t)(amount + offset);
		self->stat.st_blocks = ROUND_UP(self->stat.st_size, self->stat.st_blksize);
	}

	written = amount;

fail:
	spin_unlock(&self->lock);
	return written;
}

static TmpHandle* devtmpfs_handle_new(FileSystem* fs, mode_t mode)
{
	TmpHandle* result = handle_new(sizeof(TmpHandle));
	// If allocation failed, don't try to allocate anything else.
	if (result == NULL)
		return NULL;

	// If the file is a regular file, allocate memory for it.
	if (S_ISREG(mode))
	{
		result->buffer_cap = arch_page_size;
		result->buffer = kmalloc(result->buffer_cap);
	}

	// Set stat.
	result->handle.stat.st_size = 0;
	result->handle.stat.st_blocks = 0;
	result->handle.stat.st_blksize = 512;
	result->handle.stat.st_dev = device_id++;
	result->handle.stat.st_ino = inode_counter++;
	result->handle.stat.st_mode = mode;
	result->handle.stat.st_nlink = 1;

	// Set callbacks.
	result->handle.read = devtmpfs_handle_read;
	result->handle.write = devtmpfs_handle_write;
	result->handle.ioctl = NULL;

	return result;
}

static VfsNode* devtmpfs_hard_link(FileSystem* self, VfsNode* parent, const char* name, VfsNode* target)
{
	// TODO
	return NULL;
}

static VfsNode* devtmpfs_sym_link(FileSystem* self, VfsNode* parent, const char* name, const char* target)
{
	// TODO
	return NULL;
}

static VfsNode* devtmpfs_create(FileSystem* self, VfsNode* parent, const char* name, mode_t mode)
{
	VfsNode* result = NULL;
	TmpHandle* handle = NULL;

	// Create a new node.
	result = vfs_node_new(&devtmpfs, parent, name, S_ISDIR(mode));
	if (result == NULL)
		goto fail;

	handle = devtmpfs_handle_new(&devtmpfs, mode);
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

static VfsNode* devtmpfs_root = NULL;

static VfsNode* devtmpfs_mount(VfsNode* mount_point, const char* name, VfsNode* source)
{
	devtmpfs_root = devtmpfs.create(&devtmpfs, mount_point, name, 0644 | S_IFDIR);
	kassert(devtmpfs_root != NULL, "Couldn't create devtmpfs!\n");

	return devtmpfs_root;
}

static FileSystem devtmpfs = {
	.name = "devtmpfs",
	.mount = devtmpfs_mount,
	.populate = NULL,
	.create = devtmpfs_create,
	.hard_link = devtmpfs_hard_link,
	.sym_link = devtmpfs_sym_link,
};

void devtmpfs_register_default()
{
	// Add /dev/null, /dev/full and /dev/zero
	Handle* null = handle_new(sizeof(Handle));
	null->read = null_read;
	null->write = null_write;
	null->stat.st_size = 0;
	null->stat.st_blocks = 0;
	null->stat.st_blksize = arch_page_size;
	null->stat.st_rdev = handle_new_device();
	null->stat.st_mode = 0666 | S_IFCHR;
	devtmpfs_add_device(null, "null");

	Handle* full = handle_new(sizeof(Handle));
	full->read = full_read;
	full->write = full_write;
	full->stat.st_size = 0;
	full->stat.st_blocks = 0;
	full->stat.st_blksize = arch_page_size;
	full->stat.st_rdev = handle_new_device();
	full->stat.st_mode = 0666 | S_IFCHR;
	devtmpfs_add_device(full, "full");

	Handle* zero = handle_new(sizeof(Handle));
	zero->read = zero_read;
	zero->write = zero_write;
	zero->stat.st_size = 0;
	zero->stat.st_blocks = 0;
	zero->stat.st_blksize = arch_page_size;
	zero->stat.st_rdev = handle_new_device();
	zero->stat.st_mode = 0666 | S_IFCHR;
	devtmpfs_add_device(zero, "zero");
}

i32 devtmpfs_init()
{
	return vfs_fs_register(&devtmpfs);
}

bool devtmpfs_add_device(Handle* device, const char* name)
{
	// We're a bit early, the VFS isn't set up yet.
	if (vfs_get_root() == NULL)
		return false;

	VfsNode* node = vfs_get_node(devtmpfs_root, name, false);
	// Already have a node with this name, so fail.
	if (node != NULL)
	{
		thread_set_errno(EEXIST);
		return false;
	}

	node = vfs_node_new(&devtmpfs, devtmpfs_root, name, false);
	if (node == NULL)
	{
		print_log("vfs: Failed to add new devtmpfs node \"%s\"!\n", name);
		return false;
	}

	node->handle = device;

	device->stat.st_dev = device_id;
	device->stat.st_ino = inode_counter++;
	device->stat.st_nlink = 1;

	spin_lock(&vfs_lock);
	hashmap_insert(&devtmpfs_root->children, name, strlen(name), node);
	char path[256];
	vfs_get_path(node, path, 256);
	spin_unlock(&vfs_lock);

	return true;
}
