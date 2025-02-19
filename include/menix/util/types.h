// Commonly used types

#pragma once

#include <bits/arch_defs.h>

#define NULL				   ((void*)0)
#define offsetof(type, member) ((usize)(&((type*)0)->member))
#define atomic				   _Atomic

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
#if ARCH_BITS == 64
// Signed 64-bit integer.
typedef signed long i64;
// Unsigned 64-bit integer.
typedef unsigned long u64;
#endif

// usize
#if ARCH_BITS == 32
typedef u32 usize;
typedef i32 isize;
#elif ARCH_BITS == 64
typedef u64 usize;
typedef i64 isize;
#else
#error "Incorrect ARCH_BITS size!"
#endif

static_assert(sizeof(usize) == sizeof(void*), "usize must be the same size as a pointer!");

// Use the processor word size so we can squeeze as many bits as possible into one variable.
typedef usize Bits;

// Represents a physical address.
typedef usize PhysAddr;

// Represents a virtual address not ready for access.
typedef usize VirtAddr;

// A fixed length buffer.
typedef struct
{
	usize length;
	void* data;
} Buffer;
