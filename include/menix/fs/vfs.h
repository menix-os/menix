// Virtual File System

#pragma once
#include <menix/common.h>
#include <menix/fs/fs.h>
#include <menix/resource.h>

// A single node in the VFS.
typedef struct VfsNode
{
	Resource* resource;	   // Resource associated with this node.
	FileSystem* fs;		   // The filesystem controlling this node.
	VfsNode* parent;	   // Parent node.
	VfsNode* mount;		   // Location where this node is mounted to.
	VfsNode* children;	   // Children of this node. TODO: Use a hashmap to speed accesses.
	usize num_children;	   // Amount of children in this node.
	char* sym_link;		   // If not null: The location where this node points to as a symbolic link.
	char* name;			   // The name of the node.
	bool populated;
} VfsNode;

// Initializes the virtual file system.
void vfs_init();

// Registers a filesystem to the VFS. Returns true if successful.
bool vfs_register_fs();

// Gets the root VFS node ("/").
VfsNode* vfs_get_root();

// Create a new VFS node.
// `fs`: (Optional) The file system that owns this node.
// `parent`: (Optional) The parent of the to be created node.
// `name`: Name of the node.
// `is_dir`: True, if the node is going to be a directory.
VfsNode* vfs_node_new(FileSystem* fs, VfsNode* parent, const char* name, bool is_dir);

// Mounts a filesystem on a node. Returns true if successful.
// `parent`: The node to mount on.
// `src_path`: (Optional) The source of the
// `fs_name`: Name of the filesystem to use.
bool vfs_mount(VfsNode* parent, const char* src_path, const char* dest_path, const char* fs_name);

// Gets a VFS node from a path.
// `parent`: The node from which to start searching.
// `path`: The path to evaluate.
// `follow_links`: Whether or not to follow symbolic links.
VfsNode* vfs_get_node(VfsNode* parent, const char* path, bool follow_links);

// Get the path to the current node. Returns the size of the path.
// `parent`: The node to get the path of.
// `buffer`: Where to write the buffer to.
// `length`: The size of the buffer in bytes.
usize vfs_get_path(VfsNode* parent, char* buffer, usize length);
