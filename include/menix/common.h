/*---------------------
Common types and macros
---------------------*/

#pragma once

#include <menix/stddef.h>
#include <menix/stdint.h>

// Attributes/Decorators
#define ATTR(x) __attribute__((x))

// Macro pasting glue
#define __GLU2(x, y) x##y
#define GLUE(x, y)	 __GLU2(x, y)
