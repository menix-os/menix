// Commonly used types

#pragma once

#include <menix/config.h>
#include <menix/util/limits.h>

#define NULL				   ((void*)0)
#define offsetof(type, member) ((usize)(&((type*)0)->member))

// Signed 8-bit integer.
typedef signed char i8;
// Unsigned 8-bit integer.
typedef unsigned char u8;
// Signed 16-bit integer.
typedef signed short i16;
// Unsigned 16-bit integer.
typedef unsigned short u16;
// Signed 32-bit integer.
typedef signed int i32;
// Unsigned 32-bit integer.
typedef unsigned int u32;
#ifdef CONFIG_64_bit
// Signed 64-bit integer.
typedef signed long long i64;
// Unsigned 64-bit integer.
typedef unsigned long long u64;
#endif

// usize
#ifdef CONFIG_64_bit
typedef u64 usize;
typedef i64 isize;
#else
typedef u32 usize;
typedef i32 isize;
#endif

// Use the processor word size so we can squeeze as many bits as possible into one variable.
typedef usize Bits;
