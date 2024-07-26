// Frame buffer operations

#include <menix/video/fb.h>

void fb_fill_pixels(FrameBuffer* fb, u8 r, u8 g, u8 b)
{
	const usize bytes_pp = fb->bpp / 8;
	const u32 pixel = (r << fb->red_shift) | (g << fb->green_shift) | (b << fb->blue_shift);

	for (usize x = 0; x < fb->width; x++)
	{
		for (usize y = 0; y < fb->height; y++)
		{
			u32* const addr = (u32*)(fb->base + (y * fb->pitch) + (x * bytes_pp));
			write32(addr, pixel);
		}
	}
}

void fb_draw_bitmap(FrameBuffer* fb, void* buf, usize xpos, usize ypos, usize width, usize height)
{
	const usize bytes_pp = fb->bpp / 8;
	const u32* pixels = buf;

	for (usize x = 0; x < width; x++)
	{
		for (usize y = 0; y < height; y++)
		{
			u32* const addr = (u32*)(fb->base + ((y + ypos) * fb->pitch) + ((x + xpos) * bytes_pp));
			u32 pixel = pixels[((y * width) + x)];
			write32(addr, pixel);
		}
	}
}
