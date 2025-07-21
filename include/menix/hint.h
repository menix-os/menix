#ifndef _MENIX_UTIL_HINT_H
#define _MENIX_UTIL_HINT_H

// This pointer comes from user memory and must be checked before being read.
#define __user		 __attribute__((address_space(1)))
#define __mmio		 __attribute__((address_space(2)))
#define __nonnull	 __attribute__((nonnull))
#define __cold		 __attribute__((cold))
#define __section(x) __attribute__((section(x)))

#define likely(x)	__builtin_expect(!!(x), 1)
#define unlikely(x) __builtin_expect(!!(x), 0)

#endif
