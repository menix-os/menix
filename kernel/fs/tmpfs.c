// tmpfs file system

#include <menix/arch.h>
#include <menix/common.h>
#include <menix/fs/fs.h>
#include <menix/fs/tmpfs.h>
#include <menix/fs/vfs.h>
#include <menix/memory/alloc.h>
#include <menix/thread/process.h>
#include <menix/thread/spin.h>

#include <errno.h>
#include <string.h>

static FileSystem tmpfs;

static dev_t device_id = 0;
static ino_t inode_counter = 0;

static isize tmpfs_handle_read(struct Handle* self, FileDescriptor* fd, void* buffer, usize amount, off_t offset)
{
	spin_acquire_force(&self->lock);

	TmpHandle* const handle = (TmpHandle*)self;
	isize total_read = amount;

	// Calculate the maximum amount of data one can actually read from the buffer.
	// If we're reading past the end of the file, subtract that from the amount read.
	if ((amount + offset) >= self->stat.st_size)
		total_read -= ((amount + offset) - self->stat.st_size);

	// Copy all data to the buffer.
	memcpy(buffer, handle->buffer + offset, total_read);

	spin_free(&self->lock);
	return total_read;
}

static isize tmpfs_handle_write(struct Handle* self, FileDescriptor* fd, const void* buffer, usize amount, off_t offset)
{
	spin_acquire_force(&self->lock);

	TmpHandle* const handle = (TmpHandle*)self;

	isize written = -1;

	if (offset + amount >= handle->buffer_cap)
	{
		usize new_capacity = handle->buffer_cap;
		if (new_capacity == 0)
			new_capacity = CONFIG_page_size;
		while (offset + amount >= new_capacity)
			new_capacity *= 2;

		void* new_data = krealloc(handle->buffer, new_capacity);
		if (new_data == NULL)
		{
			thread_errno = ENOMEM;
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
	spin_free(&self->lock);
	return written;
}

static TmpHandle* tmpfs_handle_new(FileSystem* fs, mode_t mode)
{
	TmpHandle* result = handle_new(sizeof(TmpHandle));
	// If allocation failed, don't try to allocate anything else.
	if (result == NULL)
		return NULL;

	// If the file is a regular file, allocate memory for it.
	if (S_ISREG(mode))
	{
		result->buffer_cap = CONFIG_page_size;
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
	result->handle.read = tmpfs_handle_read;
	result->handle.write = tmpfs_handle_write;
	result->handle.ioctl = NULL;

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
