// 12x8 Built-in font.

#pragma once
#include <menix/common.h>

#define FONT_WIDTH	8
#define FONT_HEIGHT 12

#define FONT_GLYPH_SIZE ((FONT_WIDTH * FONT_HEIGHT) / 8)

extern u8 builtin_font[256 * FONT_GLYPH_SIZE];
