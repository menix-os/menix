// Frame buffer management

#pragma once

#include <menix/common.h>
#include <menix/io/mmio.h>
#include <menix/thread/spin.h>

// Fixed framebuffer information that cannot change after it has been initialized.
typedef struct
{
	mmio8* mmio_base;	   // Start of memory mapped IO.
	usize mmio_len;		   // Size of memory mapped IO.
	PhysAddr phys_base;	   // Start of framebuffer memory.
	usize phys_len;		   // Size of framebuffer memory.
} FbInfoFixed;

// Framebuffer information that may change at will depending on the current mode.
typedef struct
{
	usize width;	 // Width of the visible frame in pixels.
	usize height;	 // Height of the visible frame in pixels.
	u8 bpp;			 // Amount of bits per pixel.
} FbInfoVar;

// Callback functions for modifying a framebuffer.
typedef struct FrameBuffer FrameBuffer;
typedef struct
{
	usize x, y;				// Top left corner of the area to fill.
	usize width, height;	// Width and height of the area to fill.
	u32 color;				// Color to fill the area with.
} FbFillOp;

typedef struct
{
	// Opens
	int (*open)(FrameBuffer* fb);
	usize (*fill)(FrameBuffer* fb, FbFillOp* args);
} FbFuncs;

// Stores information about a frame buffer.
struct FrameBuffer
{
	SpinLock lock;			// Access lock.
	const FbFuncs funcs;	// Functions for modifying the framebuffer.
	FbInfoFixed fixed;		// Fixed information.
	FbInfoVar var;			// Variable information.
};

// Registers a framebuffer to be visible to the kernel.
void fb_register(FrameBuffer* fb);

// Unregisters a framebuffer.
void fb_unregister(FrameBuffer* fb);

// Unregisters all previous framebuffers. This is useful when e.g. a new video card
// has been detected and needs to take control now.
void fb_unregister_all();

// Get the next available framebuffer.
FrameBuffer* fb_get_next();
