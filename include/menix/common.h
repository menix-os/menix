//? Common types and macros

#pragma once

#include <menix/stddef.h>
#include <menix/stdint.h>

// Attributes/Decorators
#define ATTR(x) __attribute__((x))

// Macro pasting glue
#define __PASTE(x)	 x
#define __GLU2(x, y) __PASTE(x)##__PASTE(y)
#define GLUE(x, y)	 __GLU2(x, y)

#define asm __asm__
