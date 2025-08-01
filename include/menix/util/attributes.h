#ifndef _MENIX_UTIL_ATTRIBUTES_H
#define _MENIX_UTIL_ATTRIBUTES_H

// Hints
#define __cold              gnu::cold
#define __hot               gnu::hot
#define __used              gnu::used
#define __unused            gnu::unused
#define __format(like, ...) gnu::format(like, __VA_ARGS__)
#define __likely(x)         __builtin_expect(!!(x), 1)
#define __unlikely(x)       __builtin_expect(!!(x), 0)
// Symbol is defined by a linker script.
#define __linker
// Symbol is defined per architecture.
#define __arch

#define __weak                  gnu::weak
#define __section(x)            gnu::section(x)
#define __packed                gnu::packed
#define __aligned(x)            gnu::aligned(x)
#define __inline                gnu::always_inline
#define __init                  __used, __section(".init.text"), __cold
#define __initdata              __used, __section(".init.data")
#define __initdata_sorted(name) __used, __section(".init.data." name)

// Only clang has these attributes, GCC will throw a warning here.
// They also don't support the C23 attribute format.
#ifdef __clang__
#define __user __attribute__((noderef, address_space(1)))
#define __mmio __attribute__((noderef, address_space(2)))
#else
#define __user
#define __mmio
#endif

#endif
