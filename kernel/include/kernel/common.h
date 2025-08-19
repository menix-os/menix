#ifndef _KERNEL_COMMON_H
#define _KERNEL_COMMON_H

#define ARRAY_SIZE(array) (sizeof(array) / sizeof(array[0]))

#define ROUND_UP(value, to)      (((value) + ((to) - 1)) / (to))
#define ALIGN_DOWN(value, align) (((value) / (align)) * (align))
#define ALIGN_UP(value, align)   (ROUND_UP(value, align) * align)

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

#define BIT(value, bit) (((value) & (1 << bit)) == (1 << bit))

#define OFFSET_OF(type, field)        __builtin_offsetof(type, field)
#define CONTAINER_OF(value, p, field) ((p*)((char*)(&(value)) + offsetof(p, field)))

#define CONCAT(A, B)  _CONCAT(A, B)
#define _CONCAT(A, B) A##B

#define UNIQUE_IDENT(ident) CONCAT(ident, __COUNTER__)

// Ensures that a type is properly defined at this point.
#define ASSERT_TYPE(type) static_assert(sizeof(type) == sizeof(type), "Type was not previously defined!")

#endif
