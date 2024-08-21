// File status checking.

#pragma once
#include <menix/common.h>

typedef struct
{
	DeviceId device;
	INodeId inode;
	NLinkId nlink;
	ModeId mode;
	UserId uid;
	GroupId gid;
	DeviceId rdev;
	usize size;
	isize block_size;
	isize blocks;
	Timestamp accessed;	   // Time of the last access.
	Timestamp modified;	   // Time of the last modification.
	Timestamp created;	   // Time of the creation.
} Status;
