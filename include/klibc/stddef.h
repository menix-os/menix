//? Kernel C library - stddef.h

#pragma once
#include <menix/config.h>

#define NULL				   ((void*)0)
#define offsetof(type, member) ((size_t)(&((type*)0)->member))

// ptrdiff_t
#ifdef CONFIG_64_bit
typedef signed long long ptrdiff_t;
#else
typedef signed long ptrdiff_t;
#endif

// wchar_t
typedef unsigned int wchar_t;

// size_t
#ifdef CONFIG_64_bit
typedef unsigned long long size_t;
#else
typedef unsigned int size_t;
#endif
