// Frame buffer operations

#include <menix/gpu/fb.h>

void fb_fill_pixels(FrameBuffer* fb, uint8_t r, uint8_t g, uint8_t b)
{
	const size_t bytes_pp = fb->bpp / 8;
	const uint32_t pixel = (r << fb->red_shift) | (g << fb->green_shift) | (b << fb->blue_shift);

	for (size_t x = 0; x < fb->width; x++)
	{
		for (size_t y = 0; y < fb->height; y++)
		{
			uint32_t* addr = (uint32_t*)(fb->base + (y * fb->pitch) + (x * bytes_pp));
			*addr = pixel;
		}
	}
}
