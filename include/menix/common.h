/*---------------------
Common types and macros
---------------------*/

#pragma once

#include <menix/stddef.h>
#include <menix/stdint.h>

// Attributes/Decorators
#define ATTR(x) __attribute__((x))

// Reserved identifier
#define intern(x) __INTERNAL__##x
// For pointer types. Shows that they are allowed to have NULL as value.
#define nullable

// Macro pasting glue
#define GLU2(x, y) x##y
#define GLUE(x, y) GLU2(x, y)

// Checks a compile-time logical expression. Can be combined with other expressions using |.
// This is technically abuse of the sizeof() operator to run a static assert at any point in code, even assignments.
#define INLINE_ASSERT(expr, message) \
	sizeof(struct { \
		static_assert(expr, message); \
		char _ghost; \
	}) & 0
