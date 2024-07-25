// Frame buffer information

#pragma once

#include <menix/common.h>

// Stores information about a frame buffer.
typedef struct
{
	void* base;		 // Base address where the buffer starts.
	usize width;	 // Width of the frame in pixels.
	usize height;	 // Height of the frame in pixels.
	usize bpp;		 // Amount of bits per pixel.
	usize pitch;	 // Length of one row of pixels.
	u8 red_size;
	u8 red_shift;
	u8 green_size;
	u8 green_shift;
	u8 blue_size;
	u8 blue_shift;
} FrameBuffer;

void fb_fill_pixels(FrameBuffer* fb, u8 r, u8 g, u8 b);
