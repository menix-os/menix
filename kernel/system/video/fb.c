// Frame buffer operations

#include <menix/memory/alloc.h>
#include <menix/system/video/fb.h>
#include <menix/util/list.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

static FrameBuffer* fb_active = NULL;
static SpinLock fb_lock = spin_new();

void fb_register(FrameBuffer* fb)
{
	spin_acquire_force(&fb_lock);
	fb_active = fb;
	spin_free(&fb_lock);
}

void fb_unregister()
{
	spin_acquire_force(&fb_lock);
	fb_active = NULL;
	spin_free(&fb_lock);
}

FrameBuffer* fb_get_active()
{
	return fb_active;
}
