// File system abstraction

#pragma once
#include <menix/abi.h>
#include <menix/common.h>

// Describes a UNIX-file permission mode.
typedef struct VfsNode VfsNode;

// Describes a file system.
typedef struct FileSystem FileSystem;
typedef struct FileSystem
{
	const char name[64];	// Name of the file system.

	// Called to mount a file system onto the VFS.
	VfsNode* (*mount)(VfsNode* mount_point, const char* name, VfsNode* source);
	// Called to populate the children of node `parent`.
	void (*populate)(FileSystem* self, VfsNode* parent);
	// Called to create a new node as a child of `parent`.
	VfsNode* (*create)(FileSystem* self, VfsNode* parent, const char* name, mode_t mode);
	// Called to create a new hard link at `parent`, pointing to `target`.
	VfsNode* (*hard_link)(FileSystem* self, VfsNode* parent, const char* name, VfsNode* target);
	// Called to create a new symbolic link at `parent`, pointing to `target`.
	VfsNode* (*sym_link)(FileSystem* self, VfsNode* parent, const char* name, const char* target);
} FileSystem;
