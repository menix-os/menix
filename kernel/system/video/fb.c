// Frame buffer operations

#include <menix/memory/alloc.h>
#include <menix/system/video/fb.h>
#include <menix/util/list.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

static FrameBuffer fb_active;
static SpinLock fb_lock = {0};

void fb_register(FrameBuffer* fb)
{
	spin_lock(&fb_lock);
	fb_active = *fb;
	spin_unlock(&fb_lock);
}

FrameBuffer* fb_get_active()
{
	return &fb_active;
}
