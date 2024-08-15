// Default implementations for framebuffer functions that assume linear memory.

#include <menix/video/fb.h>

#include <string.h>

void fb_default_fill_region(FrameBuffer* fb, FbFillRegion* args)
{
	const FbModeInfo* mode = &fb->mode;

	// For each line.
	for (usize y = 0; y < args->height; y++)
	{
		// Calculate the address to copy to.
		void* addr_dst = (void*)fb->info.mmio_base + (mode->pitch * (args->y_src + y) + (mode->cpp * args->x_src));

		// Fill the color.
		for (usize x = 0; x < args->width; x++)
		{
			u8* rgb_ptr = ((u8*)addr_dst + (x * mode->cpp));
			switch (mode->cpp)
			{
				case 2: write16(rgb_ptr, (u16)args->color);
				case 3:
					rgb_ptr[0] = args->color >> 16 & 0xFF;
					rgb_ptr[1] = args->color >> 8 & 0xFF;
					rgb_ptr[2] = args->color >> 0 & 0xFF;
					break;
				case 4: write32(rgb_ptr, (u32)args->color); break;
				default: break;
			}
		}
	}
}

void fb_default_copy_region(FrameBuffer* fb, FbCopyRegion* args)
{
	// TODO
}

void fb_default_draw_region(FrameBuffer* fb, FbDrawRegion* args)
{
	const FbModeInfo* mode = &fb->mode;

	// For each line.
	for (usize y = 0; y < args->height; y++)
	{
		// Calculate the address to copy from (No x needed since each line starts on 0).
		void* addr_src = (void*)args->data + (args->width * y);
		// Calculate the address to copy to.
		void* addr_dst = (void*)fb->info.mmio_base + (mode->pitch * (args->y_src + y)) + args->x_src;

		// Copy a single line.
		memcpy(addr_dst, addr_src, args->width);
	}
}
