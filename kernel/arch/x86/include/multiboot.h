/*--------------
Multiboot header
--------------*/

#pragma once

#include <menix/common.h>
#include <menix/stdint.h>

#define MB_FLAG_ALIGN	1 << 0
#define MB_FLAG_MEMINFO 1 << 1
#define MB_FLAG_VID		1 << 2
#define MB_FLAGS		(MB_FLAG_ALIGN | MB_FLAG_MEMINFO | MB_FLAG_VID)
#define MB_MAGIC		0x1BADB002
#define MB_CHECKSUM		-(MB_MAGIC + MB_FLAGS)

#define MB_VID_MODE	  1
#define MB_VID_WIDTH  1024
#define MB_VID_HEIGHT 768
#define MB_VID_DEPTH  32

typedef struct MENIX_ATTR(aligned(0x10)) MENIX_ATTR(packed)
{
	uint32_t magic;
	uint32_t flags;
	uint32_t checksum;
	// TODO: Address headers
	char	 pad[20];
	uint32_t vid_mode;
	uint32_t vid_width;
	uint32_t vid_height;
	uint32_t vid_depth;
} MultiBootHeader;

MENIX_ATTR(used)
MENIX_ATTR(section(".multiboot"))
static const MultiBootHeader mb_header = {
	.magic = MB_MAGIC,
	.flags = MB_FLAGS,
	.checksum = MB_CHECKSUM,

	.vid_mode = MB_VID_MODE,
	.vid_width = MB_VID_WIDTH,
	.vid_height = MB_VID_HEIGHT,
	.vid_depth = MB_VID_DEPTH,
};
