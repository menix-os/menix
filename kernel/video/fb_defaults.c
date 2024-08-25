// Default implementations for framebuffer functions that assume linear memory.

#include <menix/thread/spin.h>
#include <menix/video/fb.h>

#include <string.h>

void fb_default_fill_region(FrameBuffer* fb, FbFillRegion* args)
{
	spin_acquire_force(&fb->lock);
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
	spin_free(&fb->lock);
}

void fb_default_copy_region(FrameBuffer* fb, FbCopyRegion* args)
{
	spin_acquire_force(&fb->lock);
	const FbModeInfo* mode = &fb->mode;

	// For each line.
	for (usize y = 0; y < args->height; y++)
	{
		// Calculate the offset where to copy from.
		const usize src_offset = ((mode->pitch * (args->y_src + y)) + (mode->cpp * (args->x_src)));
		const usize dst_offset = ((mode->pitch * (args->y_dst + y)) + (mode->cpp * (args->x_dst)));

		// Copy a single line.
		memmove((void*)fb->info.mmio_base + dst_offset, (void*)fb->info.mmio_base + src_offset,
				args->width * mode->cpp);
	}
	spin_free(&fb->lock);
}

void fb_default_draw_region(FrameBuffer* fb, FbDrawRegion* args)
{
	spin_acquire_force(&fb->lock);
	const FbModeInfo* mode = &fb->mode;

	// For each line.
	for (usize y = 0; y < args->height; y++)
	{
		// Calculate the address to copy from (No x needed since each line starts on 0).
		void* addr_src = (void*)args->data + (args->width * y * mode->cpp);
		// Calculate the address to copy to.
		void* addr_dst = (void*)fb->info.mmio_base + (mode->pitch * (args->y_src + y)) + (args->x_src * mode->cpp);

		// Copy a single line.
		if (mode->cpp == sizeof(u32))
			memcpy32(addr_dst, addr_src, args->width);
		else
			memcpy(addr_dst, addr_src, args->width * mode->cpp);
	}
	spin_free(&fb->lock);
}

void fb_default_update_region(FrameBuffer* fb, FbUpdateRegion* args)
{
	spin_acquire_force(&fb->lock);
	const FbModeInfo* mode = &fb->mode;

	// For each line.
	for (usize y = 0; y < args->height; y++)
	{
		// Calculate the offset where to copy from.
		const usize offset = ((mode->pitch * (args->y_src + y)) + (mode->cpp * (args->x_src)));

		// Copy a single line.
		if (mode->cpp == sizeof(u32))
			memcpy32((void*)fb->info.mmio_base + offset, (void*)args->back_buffer + offset, args->width);
		else
			memcpy((void*)fb->info.mmio_base + offset, (void*)args->back_buffer + offset, args->width * mode->cpp);
	}
	spin_free(&fb->lock);
}
