// Default implementations for the FrameBuffer.funcs structure.

#pragma once
#include <menix/video/fb.h>

#define FB_DEFAULT_FUNCS \
	(FbFuncs) \
	{ \
		.fill_region = fb_default_fill_region, .copy_region = fb_default_copy_region, \
		.draw_region = fb_default_draw_region, .update_region = fb_default_update_region, \
	}

void fb_default_fill_region(FrameBuffer* fb, FbFillRegion* args);
void fb_default_copy_region(FrameBuffer* fb, FbCopyRegion* args);
void fb_default_draw_region(FrameBuffer* fb, FbDrawRegion* args);
void fb_default_update_region(FrameBuffer* fb, FbUpdateRegion* args);
