//? Frame buffer information

#pragma once

#include <menix/common.h>

// We only support 8 buffers to draw to.
#define FB_MAX 8

// Stores information about a frame buffer.
typedef struct
{
	void*	base;		 // Base address where the buffer starts.
	size_t	width;		 // Width of the frame in pixels.
	size_t	height;		 // Height of the frame in pixels.
	size_t	bpp;		 // Amount of bits per pixel.
	size_t	pitch;		 // Length of one row of pixels.
	uint8_t red_size;	 //
	uint8_t red_shift;
	uint8_t green_size;
	uint8_t green_shift;
	uint8_t blue_size;
	uint8_t blue_shift;
} FrameBuffer;

void fb_fill_pixels(FrameBuffer* fb, uint8_t r, uint8_t g, uint8_t b);
