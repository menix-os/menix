#ifndef _KERNEL_COMPILER_H
#define _KERNEL_COMPILER_H

#if !defined(__GNUC__) && !defined(__clang__)
#error "These attributes are only supported by GCC or Clang"
#endif

// Hints
#define __cold              gnu::cold
#define __hot               gnu::hot
#define __used              gnu::used
#define __unused            gnu::unused
#define __format(like, ...) gnu::format(like, __VA_ARGS__)
#define __likely(x)         __builtin_expect(!!(x), 1)
#define __unlikely(x)       __builtin_expect(!!(x), 0)
#define __unreachable       __builtin_unreachable

// Only clang has these hints, GCC will throw a warning here.
// They also don't support the C23 attribute format.
#ifdef __clang__
#define __user __attribute__((noderef, address_space(1)))
#define __mmio __attribute__((noderef, address_space(2)))
#else
#define __user
#define __mmio
#endif

// Attributes
#define __weak       gnu::weak
#define __section(x) gnu::section(x)
#define __packed     gnu::packed
#define __aligned(x) gnu::aligned(x)
#define __inline     gnu::always_inline
#define __atomic(x)  _Atomic(x)

#endif
