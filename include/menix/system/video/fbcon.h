#pragma once
#include <menix/common.h>

// Initializes the kernel mode framebuffer console.
void fbcon_init();

// Enables or disables fbcon writing to the framebuffer.
void fbcon_enable(bool status);
