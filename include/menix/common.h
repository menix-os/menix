//? Common types and macros

#pragma once

#include <menix/config.h>
#include <menix/types.h>

// Attributes/Decorators
#define ATTR(x) __attribute__((x))

// Macro pasting glue
#define __PASTE2(x)		x
#define __PASTE2_STR(x) #x
#define __PASTE(x)		__PASTE2(x)
#define __PASTE_STR(x)	__PASTE2_STR(x)
#define __GLU2(x, y)	__PASTE(x)##__PASTE(y)
#define __GLUE(x, y)	__GLU2(x, y)

// Gets the amount of elements in a compile time array.
#define ARRAY_SIZE(array) (sizeof(array) / sizeof(array[0]))

// Alias for inline assembly
#define asm __asm__
