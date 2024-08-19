// File system abstraction

#pragma once
#include <menix/common.h>

typedef i32 FsMode;	   // Describes a UNIX-file permission mode, e.g. 0777.
typedef struct VfsNode VfsNode;
typedef struct FileSystem FileSystem;

// Describes a file system.
typedef struct FileSystem
{
	char name[64];	  // Name of the file system.

	// Called to populate the children of node `parent`.
	void (*populate)(FileSystem* this, VfsNode* parent);
	// Called to create a new node at `parent`.
	VfsNode* (*create)(FileSystem* this, VfsNode* parent, const char* name, FsMode mode);
	// Called to create a new hard link at `parent`, pointing to `target`.
	VfsNode* (*link)(FileSystem* this, VfsNode* parent, const char* name, VfsNode* target);
	// Called to create a new symbolic link at `parent`, pointing to `target`.
	VfsNode* (*sym_link)(FileSystem* this, VfsNode* parent, const char* name, const char* target);
} FileSystem;
