// Console/Terminal IO

#pragma once
#include <menix/common.h>
#include <menix/video/fb.h>

// Initializes the console with a framebuffer.
void terminal_init();

void terminal_puts(const char* buf, u32 len);
