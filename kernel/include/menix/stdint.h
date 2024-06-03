/*-------------------------
Kernel C library - stdint.h
-------------------------*/

#pragma once

// Limits of exact-width integer types.

#define INT8_MIN	(-(1U << (8  - 1)))
#define INT16_MIN	(-(1U << (16 - 1)))
#define INT32_MIN	(-(1U << (32 - 1)))
#if (MENIX_BITS == 64)
#define INT64_MIN	(-(1U << (64 - 1)))
#endif

#define INT8_MAX	((1U << (8  - 1)) - 1)
#define INT16_MAX	((1U << (16 - 1)) - 1)
#define INT32_MAX	((1U << (32 - 1)) - 1)
#if (MENIX_BITS == 64)
#define INT64_MAX	((1U << (64 - 1)) - 1)
#endif

#define UINT8_MAX	((1U << 8 ) - 1)
#define UINT16_MAX	((1U << 16) - 1)
#define UINT32_MAX	((1U << 32) - 1)
#if (MENIX_BITS == 64)
#define UINT64_MAX	((1U << 64) - 1)
#endif

// Limits of minimum-width integer types.

#define INT_LEAST8_MIN	INT8_MIN
#define INT_LEAST16_MIN	INT16_MIN
#define INT_LEAST32_MIN	INT32_MIN
#if (MENIX_BITS == 64)
#define INT_LEAST64_MIN	INT64_MIN
#endif

#define INT_LEAST8_MAX	INT8_MAX
#define INT_LEAST16_MAX	INT16_MAX
#define INT_LEAST32_MAX	INT32_MAX
#if (MENIX_BITS == 64)
#define INT_LEAST64_MAX	INT64_MAX
#endif

#define UINT_LEAST8_MAX		UINT8_MAX
#define UINT_LEAST16_MAX	UINT16_MAX
#define UINT_LEAST32_MAX	UINT32_MAX
#if (MENIX_BITS == 64)
#define UINT_LEAST64_MAX	UINT64_MAX
#endif

// Limits of fastest minimum-width integer types.

#define INT_FAST8_MIN	INT8_MIN
#define INT_FAST16_MIN	INT16_MIN
#define INT_FAST32_MIN	INT32_MIN
#if (MENIX_BITS == 64)
#define INT_FAST64_MIN	INT64_MIN
#endif

#define INT_FAST8_MAX	INT8_MAX
#define INT_FAST16_MAX	INT16_MAX
#define INT_FAST32_MAX	INT32_MAX
#if (MENIX_BITS == 64)
#define INT_FAST64_MAX	INT64_MAX
#endif

#define UINT_FAST8_MAX	UINT8_MAX
#define UINT_FAST16_MAX	UINT16_MAX
#define UINT_FAST32_MAX	UINT32_MAX
#if (MENIX_BITS == 64)
#define UINT_FAST64_MAX	UINT64_MAX
#endif

// Exact-width integer types.

typedef signed char		int8_t;
typedef unsigned char	uint8_t;
typedef signed short	int16_t;
typedef unsigned short	uint16_t;
typedef signed int		int32_t;
typedef unsigned int	uint32_t;
#if (MENIX_BITS == 64)
typedef signed long long	int64_t;
typedef unsigned long long	uint64_t;
#endif

// Minimum-width integer types.

typedef int8_t		int_least8_t;
typedef uint8_t		uint_least8_t;
typedef int16_t		int_least16_t;
typedef uint16_t	uint_least16_t;
typedef int32_t		int_least32_t;
typedef uint32_t	uint_least32_t;
#if (MENIX_BITS == 64)
typedef int64_t		int_least64_t;
typedef uint64_t	uint_least64_t;
#endif

// Fastest minimum-width integer types.

typedef int8_t		int_fast8_t;
typedef uint8_t		uint_fast8_t;
typedef int16_t		int_fast16_t;
typedef uint16_t	uint_fast16_t;
typedef int32_t		int_fast32_t;
typedef uint32_t	uint_fast32_t;
#if (MENIX_BITS == 64)
typedef int64_t		int_fast64_t;
typedef uint64_t	uint_fast64_t;
#endif

// Integer types capable of holding object pointers.

#if (MENIX_BITS == 64)
typedef int64_t		intptr_t;
typedef uint64_t	uintptr_t;
#else
typedef int32_t		intptr_t;
typedef uint32_t	uintptr_t;
#endif

// Greatest-width integer types.

#if (MENIX_BITS == 64)
typedef int64_t		intmax_t;
typedef uint64_t	uintmax_t;
#else
typedef int32_t		intmax_t;
typedef uint32_t	uintmax_t;
#endif
