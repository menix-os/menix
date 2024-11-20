// Virtual File System

#include <menix/abi/errno.h>
#include <menix/fs/devtmpfs.h>
#include <menix/fs/tmpfs.h>
#include <menix/fs/vfs.h>
#include <menix/memory/alloc.h>
#include <menix/system/arch.h>
#include <menix/system/sch/process.h>
#include <menix/util/hash_map.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

#include <string.h>

typedef struct
{
	VfsNode* target;
	VfsNode* parent;
	char* name;
} VfsPathToNode;

SpinLock vfs_lock = spin_new();
static VfsNode* vfs_root = NULL;
static HashMap(FileSystem*) fs_map;

void vfs_init()
{
	// Create root.
	vfs_root = vfs_node_new(NULL, NULL, "", false);

	// Allocate a hashmap for file systems.
	hashmap_init(fs_map, 128);

	vfs_log("Initialized virtual file system.\n");
	tmpfs_init();
	devtmpfs_init();

	// Create the root directory.
	vfs_mount(vfs_root, NULL, "/", "tmpfs");

	// Create /boot.
	vfs_node_add(vfs_root, "/boot", 0755 | S_IFDIR);
	kassert(vfs_mount(vfs_root, NULL, "/boot", "tmpfs"), "Mount failed, tmpfs unavailable!");

	// Create /tmp.
	vfs_node_add(vfs_root, "/tmp", 0755 | S_IFDIR);
	kassert(vfs_mount(vfs_root, NULL, "/tmp", "tmpfs"), "Mount failed, tmpfs unavailable!");

	// Create /dev.
	vfs_node_add(vfs_root, "/dev", 0755 | S_IFDIR);
	kassert(vfs_mount(vfs_root, NULL, "/dev", "devtmpfs"), "Mount failed, devtmpfs unavailable!");
	devtmpfs_register_default();
}

VfsNode* vfs_get_root()
{
	return vfs_root;
}

i32 vfs_fs_register(FileSystem* fs)
{
	spin_lock(&vfs_lock);
	hashmap_insert(&fs_map, fs->name, strlen(fs->name), fs);
	spin_unlock(&vfs_lock);
	vfs_log("Registered new file system \"%s\"!\n", fs->name);
	return 0;
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
	const usize name_len = strlen(name) + 1;
	node->name = kmalloc(name_len);
	memcpy(node->name, name, name_len);

	return node;
}

bool vfs_node_delete(VfsNode* node)
{
	spin_lock(&vfs_lock);

	// TODO: Traverse all child nodes.

	spin_unlock(&vfs_lock);
	return node;
}

bool vfs_create_dots(VfsNode* current, VfsNode* parent)
{
	if (current == NULL || parent == NULL)
		return false;

	// `.` links to the current directory.
	VfsNode* dot = vfs_node_new(parent->fs, current, ".", false);
	dot->hard_link = current;

	// `..` links to the parent directory.
	VfsNode* dot2 = vfs_node_new(parent->fs, parent, "..", false);
	dot2->hard_link = parent;

	// Add the nodes to the children of the target node.
	hashmap_insert(&current->children, ".", 1, dot);
	hashmap_insert(&current->children, "..", 2, dot2);

	return true;
}

// Populates the children of a `directory` using the file system.
static bool vfs_populate(VfsNode* directory)
{
	if (directory == NULL)
		return false;

	if (directory->fs && directory->fs->populate && directory->populated == false && directory->handle &&
		S_ISDIR(directory->handle->stat.st_mode))
	{
		directory->fs->populate(directory->fs, directory);
		return directory->populated;
	}
	return true;
}

static VfsPathToNode vfs_parse_path(VfsNode* parent, const char* path)
{
	if (path == NULL || strlen(path) == 0)
	{
		thread_set_errno(ENOENT);
		return (VfsPathToNode) {NULL, NULL, NULL};
	}

	// If the path ends with '/' we ask for a directory.
	const usize path_len = strlen(path);
	bool path_is_dir = path[path_len - 1] == '/';

	// Determine if we should use the parent node or start from the root.
	usize i = 0;
	VfsNode* current_node = vfs_resolve_node(parent, false);
	if (!vfs_populate(current_node))
		return (VfsPathToNode) {NULL, NULL, NULL};

	if (path[i] == '/')
	{
		current_node = vfs_resolve_node(vfs_root, false);
		while (path[i] == '/')
		{
			if (i == path_len - 1)
				return (VfsPathToNode) {current_node, current_node, strdup("/")};
			i++;
		}
	}

	while (true)
	{
		const char* elem = &path[i];
		usize part_length = 0;

		// Get the size of the current path node (until we hit a seperator).
		while (i < path_len && path[i] != '/')
		{
			part_length++;
			i++;
		}

		// Skip all further occurences of '/'.
		while (i < path_len && path[i] == '/')
			i++;

		// If the current node is the last part of the path.
		bool last = (i == path_len);

		// Copy the part of the name.
		char* elem_str = kzalloc(part_length + 1);
		memcpy(elem_str, elem, part_length);

		current_node = vfs_resolve_node(current_node, false);
		VfsNode* new_node;
		// If there is no such child in the current directory, return only the current directory instead.
		if (!hashmap_get(&current_node->children, new_node, elem_str, strlen(elem_str)))
		{
			if (last)
				return (VfsPathToNode) {NULL, current_node, elem_str};
			thread_set_errno(ENOENT);
			return (VfsPathToNode) {NULL, NULL, NULL};
		}

		new_node = vfs_resolve_node(new_node, false);
		if (!vfs_populate(new_node))
			return (VfsPathToNode) {NULL, NULL, NULL};

		if (last)
		{
			if (path_is_dir && !S_ISDIR(new_node->handle->stat.st_mode))
			{
				thread_set_errno(ENOTDIR);
				return (VfsPathToNode) {NULL, current_node, elem_str};
			}
			return (VfsPathToNode) {new_node, current_node, elem_str};
		}

		current_node = new_node;

		if (S_ISLNK(current_node->handle->stat.st_mode))
		{
			VfsPathToNode r = vfs_parse_path(current_node->parent, current_node->sym_link);
			if (r.target == NULL)
				return (VfsPathToNode) {NULL, NULL, NULL};
			current_node = r.target;
		}

		if (!S_ISDIR(current_node->handle->stat.st_mode))
		{
			thread_set_errno(ENOTDIR);
			return (VfsPathToNode) {NULL, NULL, NULL};
		}
	}

	thread_set_errno(ENOENT);
	return (VfsPathToNode) {NULL, NULL, NULL};
}

VfsNode* vfs_resolve_node(VfsNode* node, bool follow_links)
{
	// If no node is given, we can't resolve anything.
	if (node == NULL)
		return NULL;

	// If the node has a hard link, use that to get the target.
	if (node->hard_link != NULL)
		return vfs_resolve_node(node->hard_link, follow_links);

	// If the node is directly mounted somewhere, use that to get the target.
	if (node->mount != NULL)
		return vfs_resolve_node(node->mount, follow_links);

	// If the node is a symbolic link, parse the path and resolve that.
	if (node->sym_link != NULL && follow_links)
	{
		VfsPathToNode parsed = vfs_parse_path(node->parent, node->sym_link);
		// If the path didn't point to a valid node, return NULL.
		if (parsed.target == NULL)
			return NULL;
		return vfs_resolve_node(parsed.target, true);
	}

	// If it's just a regular node, we don't have to do anything.
	return node;
}

VfsNode* vfs_node_add(VfsNode* parent, const char* name, mode_t mode)
{
	spin_lock(&vfs_lock);

	VfsNode* node = NULL;
	VfsPathToNode parsed = vfs_parse_path(parent, name);

	if (parsed.parent == NULL)
		goto leave;

	if (parsed.target != NULL)
	{
		thread_set_errno(EEXIST);
		goto leave;
	}

	FileSystem* const target_fs = parsed.parent->fs;

	// Create a new node.
	VfsNode* target_node = target_fs->create(target_fs, parsed.parent, parsed.name, mode);

	// Insert this node into the parent's children list.
	hashmap_insert(&parsed.parent->children, parsed.name, strlen(parsed.name), target_node);

	// If we created a directory, also create '.' and '..' entries for navigation.
	if (S_ISDIR(target_node->handle->stat.st_mode))
		vfs_create_dots(target_node, parsed.parent);

	node = target_node;

leave:
	if (parsed.name != NULL)
		kfree(parsed.name);
	spin_unlock(&vfs_lock);
	return node;
}

bool vfs_mount(VfsNode* parent, const char* src_path, const char* dest_path, const char* fs_name)
{
	bool result = false;
	VfsPathToNode parsed = {0};
	VfsNode* source_node = NULL;

	spin_lock(&vfs_lock);

	// Check if the file system has been registered.
	FileSystem* fs;
	if (!hashmap_get(&fs_map, fs, fs_name, strlen(fs_name)))
	{
		vfs_log("Unable to mount file system \"%s\": Not previously registered!\n", fs_name);
		thread_set_errno(ENODEV);
		goto leave;
	}

	if (src_path != NULL && strlen(src_path) != 0)
	{
		parsed = vfs_parse_path(parent, src_path);
		source_node = parsed.target;
		if (source_node == NULL)
			goto leave;
		if (S_ISDIR(source_node->handle->stat.st_mode))
		{
			thread_set_errno(EISDIR);
			goto leave;
		}
	}

	parsed = vfs_parse_path(parent, dest_path);

	if (parsed.target == NULL)
		goto leave;

	if (parsed.target != vfs_root && !S_ISDIR(parsed.target->handle->stat.st_mode))
	{
		thread_set_errno(EISDIR);
		goto leave;
	}

	VfsNode* mount_node = fs->mount(parsed.parent, parsed.name, source_node);
	if (mount_node == NULL)
	{
		vfs_log("Mounting \"%s\" failed!\n", dest_path);
		goto leave;
	}

	parsed.target->mount = mount_node;
	vfs_create_dots(mount_node, parsed.parent);

	if (src_path != NULL && strlen(src_path) != 0)
		vfs_log("Mounted \"%s\" on \"%s\" with file system \"%s\".\n", src_path, dest_path, fs_name);
	else
		vfs_log("Mounted new file system \"%s\" on \"%s\".\n", fs_name, dest_path);

	// Success.
	result = true;

leave:
	if (parsed.name != NULL)
		kfree(parsed.name);
	spin_unlock(&vfs_lock);
	return result;
}

VfsNode* vfs_sym_link(VfsNode* parent, const char* path, const char* target)
{
	spin_lock(&vfs_lock);

	VfsNode* result = NULL;

	// Parse the path.
	VfsPathToNode parsed = vfs_parse_path(parent, path);

	// Node doesn't exist in the tree.
	if (parsed.parent == NULL)
		goto leave;

	// Target already exists!
	if (parsed.target != NULL)
	{
		thread_set_errno(EEXIST);
		goto leave;
	}

	FileSystem* const target_fs = parsed.parent->fs;
	VfsNode* source_node = target_fs->sym_link(target_fs, parsed.parent, parsed.name, target);
	// Add the symbolic link to the parent's children list.
	hashmap_insert(&parsed.parent->children, parsed.name, strlen(parsed.name), source_node);

	result = source_node;
leave:
	spin_unlock(&vfs_lock);
	return result;
}

usize vfs_get_path(VfsNode* target, char* buffer, usize length)
{
	if (target == NULL)
		return 0;

	usize offset = 0;
	if (target->parent != vfs_root && target->parent != NULL)
	{
		VfsNode* parent = vfs_resolve_node(target->parent, false);

		if (parent != vfs_root && parent != NULL)
		{
			offset += vfs_get_path(parent, buffer, length - offset - 1);
			buffer[offset++] = '/';
		}
	}

	if (memcmp(target->name, "/", 1) != 0)
	{
		memcpy(buffer + offset, target->name, length - offset);
		return strlen(target->name) + offset;
	}
	return offset;
}

VfsNode* vfs_get_node(VfsNode* parent, const char* path, bool follow_links)
{
	// spin_lock(&vfs_lock);

	VfsNode* ret = NULL;

	VfsPathToNode r = vfs_parse_path(parent, path);
	if (r.target == NULL)
		goto leave;

	if (follow_links)
	{
		ret = vfs_resolve_node(r.target, true);
		goto leave;
	}

	ret = r.target;

leave:
	if (r.name != NULL)
		kfree(r.name);
	// spin_unlock(&vfs_lock);
	return ret;
}
