// Frame buffer management

#pragma once

#include <menix/common.h>
#include <menix/io/mmio.h>
#include <menix/system/abi.h>
#include <menix/system/driver.h>
#include <menix/util/spin.h>

// Fixed framebuffer information that cannot change after it has been initialized.
typedef struct
{
	mmio8* mmio_base;	   // Start of memory mapped IO.
	usize mmio_len;		   // Size of memory mapped IO.
	PhysAddr phys_base;	   // Start of framebuffer memory.
	usize phys_len;		   // Size of framebuffer memory.
} FbBufferInfo;

typedef struct
{
	u32 offset;			// Shift offset in bits.
	u32 size;			// Size in bits.
	bool big_endian;	// True, if the most significant bit is first.
} FbColorBits;

// Framebuffer information that may change at will depending on the current mode.
typedef struct
{
	u32 width, height;						// Resolution of the visible frame in pixels.
	u32 v_width, v_height;					// Resolution of the virtual frame in pixels.
	u32 v_off_x, v_off_y;					// Offset from virtual to visible resolution.
	u8 cpp;									// Amount of bytes per pixel.
	u32 pitch;								// Length of a line in bytes.
	FbColorBits red, green, blue, alpha;	// Bitfields for each part of a pixel.
} FbModeInfo;

// Arguments passed to `FbFuncs.fill_region`.
typedef struct
{
	u32 x_src, y_src;	  // Top left corner of the area to fill.
	u32 width, height;	  // Width and height of the area to fill.
	u32 color;			  // Color to fill the area with.
} FbFillRegion;

// Arguments passed to `FbFuncs.copy_region`.
typedef struct
{
	u32 x_src, y_src;	  // Top left corner of the area to copy.
	u32 x_dst, y_dst;	  // Top left corner of where to copy the area to.
	u32 width, height;	  // Width and height of the area to copy.
} FbCopyRegion;

// Arguments passed to `FbFuncs.draw_region`.
typedef struct
{
	u32 x_src, y_src;	  // Top left corner of the framebuffer to draw to.
	u32 width, height;	  // Width and height of the image to draw.
	const u8* data;		  // Pointer to the image data.
} FbDrawRegion;

// Arguments passed to `FbFuncs.update_region`.
typedef struct
{
	u32 x_src, y_src;		  // Top left corner of the region to update.
	u32 width, height;		  // Width and height of the region to update.
	const u8* back_buffer;	  // Start address of the back buffer.
} FbUpdateRegion;

typedef struct FrameBuffer FrameBuffer;
// Callback functions for modifying a framebuffer.
typedef struct
{
	// Sets the video mode. Returns 0 on success.
	i32 (*set_mode)(FrameBuffer* fb);
	// Opens the framebuffer for writing for a user. Returns 0 on success.
	i32 (*open)(FrameBuffer* fb, uid_t user);
	// Releases the framebuffer. Returns 0 on success.
	i32 (*release)(FrameBuffer* fb, uid_t user);
	// Fills a rectangular region with a single color.
	void (*fill_region)(FrameBuffer* fb, FbFillRegion* args);
	// Copies a rectangular region from one location to another.
	void (*copy_region)(FrameBuffer* fb, FbCopyRegion* args);
	// Draws an image to a location.
	void (*draw_region)(FrameBuffer* fb, FbDrawRegion* args);
	// Updates modified regions from a back buffer of the same layout and size.
	void (*update_region)(FrameBuffer* fb, FbUpdateRegion* args);
} FbFuncs;

// Stores information about a framebuffer.
struct FrameBuffer
{
	SpinLock lock;		  // Access lock.
	Device* parent;		  // The device owning this framebuffer.
	FbFuncs funcs;		  // Functions for modifying the framebuffer, set by a driver.
	FbBufferInfo info;	  // Fixed information about how to access the frame buffer.
	FbModeInfo mode;	  // Information about the current video mode.
};

// Registers a framebuffer as the active rendering target.
void fb_register(FrameBuffer* fb);

// Unregisters the active framebuffer.
// This is useful when e.g. a new video card has been detected.
void fb_unregister();

// Get the active framebuffer.
FrameBuffer* fb_get_active();
