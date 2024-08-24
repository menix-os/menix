// Common types and macros

#pragma once

#include <menix/config.h>
#include <menix/util/types.h>
#include <menix/util/units.h>

// Attributes/Decorators
#define ATTR(...) __attribute__((__VA_ARGS__))

// Macro pasting glue
#define __PASTE2(x)		x
#define __PASTE2_STR(x) #x
#define __PASTE(x)		__PASTE2(x)
#define __PASTE_STR(x)	__PASTE2_STR(x)
#define __GLU2(x, y)	__PASTE(x)##__PASTE(y)
#define __GLUE(x, y)	__GLU2(x, y)

// Gets the amount of elements in a compile time array.
#define ARRAY_SIZE(array) (sizeof(array) / sizeof(array[0]))

// Alias for inline assembly.
#define asm __asm__

// Align an integer, rounding down.
#define ALIGN_DOWN(value, align) ((value) & ~((typeof(value))((align) - 1)))

// Align an integer, rounding up.
#define ALIGN_UP(value, align) (ALIGN_DOWN(value, align) + align)

#define ROUND_UP(value, to) (((value) + ((to) - 1)) / (to))

// Uses the smaller of the two value.
#define MIN(a, b) ((a) < (b) ? (a) : (b))
