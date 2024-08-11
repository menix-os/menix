// Terminal Output

#include <menix/io/serial.h>
#include <menix/io/terminal.h>
#include <menix/util/builtin_font.h>
#include <menix/video/fb.h>

#include <string.h>

static FrameBuffer* internal_fb;

static usize ch_width;
static usize ch_height;
static usize ch_xpos;
static usize ch_ypos;

void terminal_init()
{
	internal_fb = fb_get_next();
	ch_width = internal_fb->var.width / FONT_WIDTH;
	ch_height = internal_fb->var.height / FONT_HEIGHT;
	ch_xpos = 0;
	ch_ypos = 0;
}

// Moves all lines up by one line.
static void terminal_scroll()
{
	volatile void* const buf = internal_fb->fixed.mmio_base;
	const usize offset =
		(FONT_HEIGHT * internal_fb->var.width * (internal_fb->var.bpp / 8));	// Offset for 1 line of characters.
	for (usize y = 1; y < ch_height; y++)
	{
		memcpy((void*)buf + (offset * (y - 1)), (void*)buf + (offset * y), offset);
	}
}

void terminal_putchar(u32 ch)
{
	// Output to serial first.
	serial_putchar(ch);

	// Then update the framebuffer console if there is one.
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

	u32* buf = (u32*)(internal_fb->fixed.mmio_base);
	const u8 c = (u8)ch;

	// Plot the character.
	const usize pix_xpos = ch_xpos * FONT_WIDTH;
	const usize pix_ypos = ch_ypos * FONT_HEIGHT;
	for (usize y = 0; y < FONT_HEIGHT; y++)
	{
		for (usize x = 0; x < FONT_WIDTH; x++)
		{
			u32* const pixel = buf + (((pix_ypos + y) * internal_fb->var.width) + (pix_xpos + x));
			*pixel = builtin_font[(c * FONT_GLYPH_SIZE) + y] & (1 << (FONT_WIDTH - x - 1)) ? 0xFFFFFFFF : 0xFF000000;
		}
	}
	ch_xpos++;
}
