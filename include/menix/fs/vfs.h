// Virtual File System

#pragma once
#include <menix/common.h>
#include <menix/fs/fs.h>
#include <menix/fs/handle.h>
#include <menix/thread/spin.h>
#include <menix/util/hash_map.h>

#define vfs_log(fmt, ...) kmesg("[VFS]\t" fmt, ##__VA_ARGS__)

// A single node in the VFS.
typedef struct VfsNode
{
	Handle* handle;				   // Handle associated with this node.
	FileSystem* fs;				   // The filesystem controlling this node.
	VfsNode* parent;			   // Parent node.
	VfsNode* mount;				   // Location where this node is mounted to.
	HashMap(VfsNode*) children;	   // Children of this node.
	VfsNode* hard_link;			   // If not null: The location where this node points to as a hard link.
	char* sym_link;				   // If not null: The location where this node points to as a symbolic link.
	char* name;					   // The name of the node.
	bool populated;				   // True if the children of this node have been populated.
} VfsNode;

extern SpinLock vfs_lock;

// Initializes the virtual file system.
void vfs_init();

// Registers a filesystem to the VFS. Returns 0 if successful.
i32 vfs_fs_register(FileSystem* fs);

// Gets the root VFS node ("/").
VfsNode* vfs_get_root();

// Creates a new VFS node. This function is meant to be called by a file system driver.
// If you want to create a new standalone node, use vfs_node_add
// `fs`: The file system that owns this node.
// `parent`: The parent of the to be created node.
// `name`: Name of the node.
// `is_dir`: True, if the node is going to be a directory.
VfsNode* vfs_node_new(FileSystem* fs, VfsNode* parent, const char* name, bool is_dir);

// Creates a new VFS node.
// `parent`: The parent of the to be created node.
// `name`: Name of the node.
// `mode`: The stat mode of the node.
VfsNode* vfs_node_add(VfsNode* parent, const char* name, mode_t mode);

// Destroys a VFS node. Returns true upon success.
// `parent`: The parent of the to be created node.
// `name`: Name of the node.
// `is_dir`: True, if the node is going to be a directory.
bool vfs_node_delete(VfsNode* node);

// Mounts a filesystem on a node. Returns true upon success.
// `parent`: Node relative to dest_path.
// `src_path`: (Optional) The source of the block device to mount.
// `dest_path`: The path where to mount the file system.
// `fs_name`: Name of the filesystem to use.
bool vfs_mount(VfsNode* parent, const char* src_path, const char* dest_path, const char* fs_name);

// Get the path to the current node. Returns the size of the bytes written.
// `target`: The node to get the path of.
// `buffer`: Where to write the buffer to.
// `length`: The size of the buffer in bytes.
usize vfs_get_path(VfsNode* target, char* buffer, usize length);

// Create the `.` and `..` entries at `parent`. Returns true upon success.
// `current`: The current directory.
// `parent`: The previous directory.
bool vfs_create_dots(VfsNode* current, VfsNode* parent);

// Creates a symbolic link.
// `parent`: The node to base a relative path off of.
// `path`: The path where to create the new node.
// `target`: Where the symbolic link is supposed to point to.
VfsNode* vfs_sym_link(VfsNode* parent, const char* path, const char* target);

// Gets the actual node pointed to by `node` if it links somewhere else.
// `node`: The node to resolve.
// `follow_links`: Whether or not to follow symbolic links.
VfsNode* vfs_resolve_node(VfsNode* node, bool follow_links);

// Gets a node pointer to by a `path`.
VfsNode* vfs_get_node(VfsNode* parent, const char* path, bool follow_links);
