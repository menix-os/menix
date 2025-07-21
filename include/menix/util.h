#ifndef _MENIX_UTIL_H
#define _MENIX_UTIL_H

#define likely(x)	__builtin_expect(!!(x), 1)
#define unlikely(x) __builtin_expect(!!(x), 0)

#define array_size(array) (sizeof(array) / sizeof(array[0]))

#define round_up(value, to)		 (((value) + ((to) - 1)) / (to))
#define align_down(value, align) (((value) / (align)) * (align))
#define align_up(value, align)	 (round_up(value, align) * align)

#define min(a, b) ((a) < (b) ? (a) : (b))
#define max(a, b) ((a) > (b) ? (a) : (b))

#define bit(value, bit) (((value) & (1 << bit)) == (1 << bit))

#define offsetof(type, field)		  __builtin_offsetof(type, field)
#define container_of(value, p, field) ((p*)((char*)(&(value)) + offsetof(p, field)))

#endif
