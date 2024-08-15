// Default implementations for the FrameBuffer.funcs structure.

#pragma once
#include <menix/video/fb.h>

void fb_default_fill_region(FrameBuffer* fb, FbFillRegion* args);
void fb_default_copy_region(FrameBuffer* fb, FbCopyRegion* args);
void fb_default_draw_region(FrameBuffer* fb, FbDrawRegion* args);
