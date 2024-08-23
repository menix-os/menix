// BMP File format structures.

#pragma once
#include <menix/common.h>

typedef struct
{
	u32 size;
	i32 width;
	i32 height;
	u16 planes;
	u16 bpp;
	u32 compression;
	u32 image_size;
	u32 horizontal_res;
	u32 vertical_res;
	u32 num_colors;
	u32 num_important_colors;
} ATTR(packed) BmpDibHeader;

typedef struct
{
	u16 header;
	u32 size;
	u16 reserved;
	u16 reserved2;
	u32 offset;
	BmpDibHeader dib;
} ATTR(packed) BmpHeader;

// Unpacks a 24bit BMP to a 32bit buffer.
void bmp_unpack24_to_32(u8* rgba, const BmpHeader* bmp);
