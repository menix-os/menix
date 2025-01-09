// Common types and macros

#pragma once

#include <menix/util/types.h>
#include <menix/util/units.h>

// Attributes/Decorators
#define ATTR(...) __attribute__((__VA_ARGS__))

// Build time
#define BUILD_TIME __TIME__ " " __DATE__

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

#define ROUND_UP(value, to) (((value) + ((to) - 1)) / (to))

// Align an integer, rounding down.
#define ALIGN_DOWN(value, align) (((value) / (align)) * (align))

// Align an integer, rounding up.
#define ALIGN_UP(value, align) (ROUND_UP(value, align) * align)

// Uses the smaller of the two value.
#define MIN(a, b) ((a) < (b) ? (a) : (b))

#define BIT(value, bit) (((value) & (1 << bit)) == (1 << bit))

// Gets the parent structure where the field is contained in.
#define parentof(parent_type, field) (offsetof(parent_type, field))

// Variables reclaimed after boot.
#define RECLAIM ATTR(section(".reclaim"))

#define restrict __restrict
