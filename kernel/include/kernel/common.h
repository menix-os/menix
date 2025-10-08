#pragma once

#define ARRAY_SIZE(array) (sizeof(array) / sizeof(array[0]))

#define ROUND_UP(value, to)      (((value) + ((to) - 1)) / (to))
#define ALIGN_DOWN(value, align) (((value) / (align)) * (align))
#define ALIGN_UP(value, align)   (ROUND_UP(value, align) * align)

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

#define BIT_TEST(value, bit) (((value) & (1 << bit)) == (1 << bit))

#define CONTAINER_OF(value, p, field) ((p*)((char*)(&(value)) + offsetof(p, field)))

#define CONCAT(A, B)  _CONCAT(A, B)
#define _CONCAT(A, B) A##B

// Creates a unique identifier for a compilation unit.
#define UNIQUE_IDENT(ident) CONCAT(__unique_##ident, __COUNTER__)

// Ensures that a type is properly defined at this point.
// This is used to ensure that e.g. architecture-specific structures are defined.
#define ASSERT_TYPE(type) static_assert(sizeof(type) == sizeof(type))
