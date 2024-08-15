// Frame buffer operations

#include <menix/log.h>
#include <menix/memory/alloc.h>
#include <menix/video/fb.h>

#define MAX_FB 16

static FrameBuffer* early_buffer = NULL;
static FrameBuffer* buffers[MAX_FB] = {0};
static usize num_buffers = 0;

void fb_set_early(FrameBuffer* fb)
{
	early_buffer = fb;
}

FrameBuffer* fb_get_early()
{
	return early_buffer;
}

void fb_register(FrameBuffer* fb)
{
	if (num_buffers >= MAX_FB)
	{
		kmesg("Failed to register framebuffer 0x%p: Already have enough buffers.\n", fb);
		return;
	}

	for (usize i = 0; i < MAX_FB; i++)
	{
		if (buffers[i] == NULL)
		{
			buffers[i] = fb;
			num_buffers += 1;
			return;
		}
	}
}

void fb_unregister(FrameBuffer* fb)
{
	for (usize i = 0; i < MAX_FB; i++)
	{
		if (buffers[i] == fb)
		{
			buffers[i] = NULL;
			num_buffers -= 1;
			return;
		}
	}
	kmesg("Failed to unregister framebuffer 0x%p: Buffer wasn't previously registered.", fb);
}

void fb_unregister_all()
{
	for (usize i = 0; i < MAX_FB; i++)
	{
		buffers[i] = NULL;
	}
}

FrameBuffer* fb_get_next()
{
	for (usize i = 0; i < MAX_FB; i++)
	{
		if (buffers[i])
			return buffers[i];
	}
	return NULL;
}
