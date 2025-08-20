#ifndef _MENIX_STDDEF_H
#define _MENIX_STDDEF_H

typedef __SIZE_TYPE__ size_t;

#define offsetof(type, member) __builtin_offsetof(type, member)

#endif
