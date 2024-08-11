// Console/Terminal IO

#pragma once
#include <menix/common.h>
#include <menix/video/fb.h>

// Initializes the console with a framebuffer.
void terminal_init();

void terminal_putchar(u32 ch);
