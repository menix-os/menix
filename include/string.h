#ifndef _STRING_H
#define _STRING_H

#include <menix/types.h>

void* memcpy(void* dest, const void* src, usize n);

#define memset __builtin_memset
#define strlen __builtin_strlen

#endif
