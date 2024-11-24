// Terminal Output

#include <menix/common.h>
#include <menix/fs/devtmpfs.h>
#include <menix/fs/handle.h>
#include <menix/io/terminal.h>
#include <menix/memory/alloc.h>
#include <menix/system/module.h>
#include <menix/system/video/fb.h>

#include <string.h>

#define FONT_WIDTH		8
#define FONT_HEIGHT		12
#define FONT_GLYPH_SIZE ((FONT_WIDTH * FONT_HEIGHT) / 8)

extern u8 fbcon_font[256 * FONT_GLYPH_SIZE];

// Internal terminal handle.
static Handle* handle;

// Internal render target.
static FrameBuffer* internal_fb;

// Back buffer.
static void* internal_buffer;

// After more than UPDATE_QUEUE_MAX changes, we force an update of the entire screen.
#define UPDATE_QUEUE_MAX 64
static FbUpdateRegion update_queue[UPDATE_QUEUE_MAX];	 // Queued changes to the frame buffer.
static usize update_count = 0;							 // Amount of queued changes.

static usize ch_width;			  // Screen width in characters
static usize ch_height;			  // Screen height in characters
static usize ch_xpos, ch_ypos;	  // Current cursor position in characters

// Copies the back buffer to the screen.
MODULE_FN void fbcon_copy_screen()
{
	const FbModeInfo* mode = &internal_fb->mode;
	memcpy((void*)internal_fb->info.mmio_base, internal_buffer, mode->pitch * mode->height);
	update_count = 0;
}

// Moves all lines up by one line.
MODULE_FN void fbcon_scroll()
{
	void* const buf = internal_buffer;
	// Offset for 1 line of characters.
	const usize offset = (FONT_HEIGHT * internal_fb->mode.pitch);

	// Move all lines up by one.
	memmove(buf, buf + offset, offset * (ch_height - 1));

	// Blank the new line.
	memset((void*)buf + (offset * (ch_height - 1)), 0x00, offset);

	fbcon_copy_screen();
}

MODULE_FN void fbcon_putchar(u32 ch)
{
	if (!internal_fb)
		return;

	const u8 c = (u8)ch;
	FbModeInfo* mode = &internal_fb->mode;

	// Plot the character.
	const usize pix_xpos = ch_xpos * FONT_WIDTH;
	const usize pix_ypos = ch_ypos * FONT_HEIGHT;
	for (usize y = 0; y < FONT_HEIGHT; y++)
	{
		for (usize x = 0; x < FONT_WIDTH; x++)
		{
			const usize offset = ((mode->pitch * (pix_ypos + y)) + (mode->cpp * (pix_xpos + x)));
			const u32 pixel =
				fbcon_font[(c * FONT_GLYPH_SIZE) + y] & (1 << (FONT_WIDTH - x - 1)) ? 0xFFFFFFFF : 0xFF000000;
			// Write to back buffer.
			mmio_write32(internal_buffer + offset, pixel);
		}
	}
	// Increment cursor.
	ch_xpos++;

	// Mark this region as modified. If we've exceeded the limit, force a redraw.
	if (update_count >= UPDATE_QUEUE_MAX)
		fbcon_copy_screen();
	else
	{
		update_queue[update_count++] = (FbUpdateRegion) {
			.back_buffer = internal_buffer,
			.x_src = pix_xpos,
			.y_src = pix_ypos,
			.width = FONT_WIDTH,
			.height = FONT_HEIGHT,
		};
	}
}

MODULE_FN isize fbcon_write(Handle* handle, FileDescriptor* fd, const void* buf, usize len, off_t offset)
{
	// Write each character to the buffer.
	for (usize i = 0; i < len; i++)
	{
		char ch = ((char*)buf)[i];
		switch (ch)
		{
			case '\b':
			{
				if (ch_xpos > 0)
					ch_xpos -= 1;
				fbcon_putchar(' ');
				ch_xpos -= 1;
				continue;
			}
			case '\n':
			{
				ch_xpos = 0;
				ch_ypos += 1;
				continue;
			}
			case '\t':
			{
				ch_xpos = ALIGN_UP(ch_xpos + 1, 8);
				continue;
			}
			case '\0': continue;
			default: break;
		}

		// Line break.
		if (ch_xpos >= ch_width)
		{
			ch_xpos = 0;
			ch_ypos += 1;
		}

		// If writing past the last line, scroll.
		if (ch_ypos >= ch_height)
		{
			fbcon_scroll();
			// Move cursor to the most bottom line.
			ch_ypos = ch_height - 1;
		}

		fbcon_putchar(ch);
	}

	// After we're done drawing, copy the modified pixels from the backbuffer.
	for (usize i = 0; i < update_count; i++)
	{
		internal_fb->funcs.update_region(internal_fb, &update_queue[i]);
	}
	update_count = 0;
	return len;
}

void fbcon_post()
{
	FrameBuffer* fb = fb_get_active();

	// If no regular framebuffer is available, we can't write to anything.
	if (fb == NULL)
		return;

	internal_fb = fb;

	// Allocate a back buffer.
	const FbModeInfo* mode = &internal_fb->mode;
	internal_buffer = kzalloc(mode->pitch * mode->height);

	ch_width = internal_fb->mode.width / FONT_WIDTH;
	ch_height = internal_fb->mode.height / FONT_HEIGHT;
	ch_xpos = 0;
	ch_ypos = 0;

	// Clear the screen.
	memset((void*)internal_fb->info.mmio_base, 0, mode->pitch * mode->height);

	module_log("Switching to framebuffer console\n");

	Handle* h = terminal_get_active_node()->handle;
	h->write = fbcon_write;

	module_log("Switched to framebuffer console on /dev/terminal%zu\n", terminal_get_active());
	module_log("Framebuffer Resolution = %ux%ux%hhu (Virtual = %ux%u)\n", internal_fb->mode.width,
			   internal_fb->mode.height, internal_fb->mode.cpp * 8, internal_fb->mode.v_width,
			   internal_fb->mode.v_height);
}

i32 fbcon_init()
{
	// Register device.
	handle = handle_new(sizeof(Handle));
	handle->write = fbcon_write;
	if (devtmpfs_add_device(handle, "fbcon") == false)
	{
		module_log("Failed to initialize fbcon!\n");
		return 1;
	}

	module_register_post(fbcon_post);
	return 0;
}

MODULE_DEFAULT(fbcon_init, NULL);
