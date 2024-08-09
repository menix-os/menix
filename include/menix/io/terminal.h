// Console/Terminal IO

#pragma once
#include <menix/common.h>
#include <menix/video/fb.h>

// Initializes the console with a framebuffer. This is optional. If `fb` is NULL,
// the console will only output to serial.
void terminal_init(FrameBuffer* fb);

void terminal_putchar(u32 ch);
