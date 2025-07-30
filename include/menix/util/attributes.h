#ifndef _MENIX_UTIL_ATTRIBUTES_H
#define _MENIX_UTIL_ATTRIBUTES_H

#define __cold            __attribute__((cold))
#define __hot             __attribute__((hot))
#define __used            __attribute__((used))
#define __unused          __attribute__((unused))
#define __weak            __attribute__((weak))
#define __noreturn        _Noreturn
#define __likely(x)       __builtin_expect(!!(x), 1)
#define __unlikely(x)     __builtin_expect(!!(x), 0)
#define __atomic          _Atomic
#define __init            __attribute__((used, section(".init.text"), cold))
#define __initdata        __attribute__((used, section(".init.data")))
#define __initdata_ord(p) __attribute__((used, section(".init.data." p)))

// Only clang has these attributes, GCC will throw a warning here.
#ifdef __clang__
#define __user __attribute__((noderef, address_space(1)))
#define __mmio __attribute__((noderef, address_space(2)))
#else
#define __user
#define __mmio
#endif

#endif
