/*---------------------
Common types and macros
---------------------*/

#pragma once

#include <menix/stddef.h>

// Attributes/Decorators
#define ATTR(x) __attribute__((x))

// Reserved identifier
#define intern(x) __INTERNAL__

// Macro pasting glue
#define GLU2(x, y) x##y
#define GLUE(x, y) GLU2(x, y)

// For pointer types. Shows that they are allowed to have NULL as value.
#define nullable
