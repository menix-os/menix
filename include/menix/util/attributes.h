#ifndef _MENIX_UTIL_ATTRIBUTES_H
#define _MENIX_UTIL_ATTRIBUTES_H

#ifdef __clang__
#define __user __attribute__((address_space(1)))
#define __mmio __attribute__((address_space(2)))
#else
#define __user
#define __mmio
#endif

#define __cold		  __attribute__((cold))
#define __hot		  __attribute__((hot))
#define __likely(x)	  __builtin_expect(!!(x), 1)
#define __unlikely(x) __builtin_expect(!!(x), 0)
#define __used		  __attribute__((used))
#define __unused	  __attribute__((unused))
#define __weak		  __attribute((weak))
#define __noreturn	  _Noreturn

#endif
