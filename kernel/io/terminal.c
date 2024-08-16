// Terminal Output

#include <menix/io/serial.h>
#include <menix/io/terminal.h>
#include <menix/memory/alloc.h>
#include <menix/util/builtin_font.h>
#include <menix/video/fb.h>

#include <string.h>

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

void terminal_init()
{
	FrameBuffer* early_fb = fb_get_early();
	FrameBuffer* regular_fb = fb_get_next();

	// If a regular framebuffer is available, use that.
	if (regular_fb != NULL)
		internal_fb = regular_fb;
	// If not, try to get an early framebuffer.
	else
		internal_fb = early_fb;

	// If that fails as well, don't write to a framebuffer.
	if (!internal_fb)
		return;

	// Allocate a back buffer.
	const FbModeInfo* mode = &internal_fb->mode;
	internal_buffer = kzalloc(mode->pitch * mode->height);

	ch_width = internal_fb->mode.width / FONT_WIDTH;
	ch_height = internal_fb->mode.height / FONT_HEIGHT;
	ch_xpos = 0;
	ch_ypos = 0;

	// Clear the screen.
	memset((void*)internal_fb->info.mmio_base, 0, mode->pitch * mode->height);
}

// Copies the back buffer to the screen.
static void copy_to_screen()
{
	const FbModeInfo* mode = &internal_fb->mode;
	memcpy((void*)internal_fb->info.mmio_base, internal_buffer, mode->pitch * mode->height);
	update_count = 0;
}

// Moves all lines up by one line.
static void terminal_scroll()
{
	void* const buf = internal_buffer;
	// Offset for 1 line of characters.
	const usize offset = (FONT_HEIGHT * internal_fb->mode.pitch);

	// Move all lines up by one.
	memmove(buf, buf + offset, offset * (ch_height - 1));

	// Blank the new line.
	memset((void*)buf + (offset * (ch_height - 1)), 0x00, offset);

	copy_to_screen();
}

static void terminal_putchar(u32 ch)
{
	serial_putchar(ch);

	if (!internal_fb)
		return;

	switch (ch)
	{
		case '\n':
		{
			ch_xpos = 0;
			ch_ypos += 1;
			return;
		}
		case '\0': return;
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
		terminal_scroll();
		// Move cursor to the most bottom line.
		ch_ypos = ch_height - 1;
	}

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
				builtin_font[(c * FONT_GLYPH_SIZE) + y] & (1 << (FONT_WIDTH - x - 1)) ? 0xFFFFFFFF : 0xFF000000;
			// Write to back buffer.
			write32(internal_buffer + offset, pixel);
		}
	}
	// Increment cursor.
	ch_xpos++;

	// Mark this region as modified. If we've exceeded the limit, force a redraw.
	if (update_count >= UPDATE_QUEUE_MAX)

		copy_to_screen();
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

void terminal_puts(const char* buf, u32 len)
{
	// Write each character to the buffer.
	for (usize i = 0; i < len; i++)
		terminal_putchar(buf[i]);

	// After we're done drawing, copy the modified pixels from the backbuffer.
	for (usize i = 0; i < update_count; i++)
	{
		internal_fb->funcs.update_region(internal_fb, &update_queue[i]);
	}
	update_count = 0;
}
