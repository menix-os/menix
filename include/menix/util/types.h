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
#if CONFIG_bits == 64
// Signed 64-bit integer.
typedef signed long long i64;
// Unsigned 64-bit integer.
typedef unsigned long long u64;
#elif CONFIG_bits == 128
// Signed 128-bit integer.
typedef __int128 i128;
// Unsigned 128-bit integer.
typedef unsigned __int128 u128;
#endif

// usize
#if CONFIG_bits == 32
typedef u32 usize;
typedef i32 isize;
#elif CONFIG_bits == 64
typedef u64 usize;
typedef i64 isize;
#elif CONFIG_bits == 128
typedef u128 usize;
typedef i128 isize;
#else
#error "Invalid word size!"
#endif

// Use the processor word size so we can squeeze as many bits as possible into one variable.
typedef usize Bits;

// Represents a physical address.
typedef usize PhysAddr;
