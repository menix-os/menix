// Bit map utilities

#pragma once
#include <menix/common.h>

// Get the bit at `bit` in `map`.
#define bitmap_get(map, bit) ((((u8*)(map))[(bit) / 8] & (1 << ((bit) % 8))) != 0)

// Enable the bit at `bit` in `map`.
#define bitmap_set(map, bit) ((u8*)(map))[(bit) / 8] |= (1 << ((bit) % 8))

// Disable the bit at `bit` in `map`.
#define bitmap_clear(map, bit) ((u8*)(map))[(bit) / 8] &= ~(1 << ((bit) % 8))

typedef u8* BitMap;
