// Frame buffer operations

#include <menix/log.h>
#include <menix/memory/alloc.h>
#include <menix/util/list.h>
#include <menix/video/fb.h>

static FrameBuffer* early_buffer = NULL;
static List(FrameBuffer*) buffers = {0};

void fb_set_early(FrameBuffer* fb)
{
	early_buffer = fb;
	// Also register as a regular framebuffer.
	list_push(&buffers, fb);
}

FrameBuffer* fb_get_early()
{
	return early_buffer;
}

void fb_register(FrameBuffer* fb)
{
	list_push(&buffers, fb);
}

void fb_unregister(FrameBuffer* fb)
{
	isize idx;
	list_find(&buffers, idx, fb);
	if (idx == -1)
	{
		kmesg("Failed to unregister framebuffer 0x%p: Buffer wasn't previously registered.", fb);
		return;
	}

	list_pop(&buffers, idx);
}

void fb_unregister_all()
{
	list_free(&buffers);
}

FrameBuffer* fb_get_next()
{
	list_iter(&buffers, buf_iter)
	{
		if (*buf_iter != NULL)
			return *buf_iter;
	}
	return NULL;
}
