#ifndef _UAPI_MENIX_TYPES_H
#define _UAPI_MENIX_TYPES_H

#ifndef __KERNEL__
#ifndef __EXPORTED_HEADERS__
#warning "Don't include this file in user programs!"
#endif
#endif

typedef __INT8_TYPE__ __i8;
typedef __UINT8_TYPE__ __u8;

typedef __INT16_TYPE__ __i16;
typedef __UINT16_TYPE__ __u16;

typedef __INT32_TYPE__ __i32;
typedef __UINT32_TYPE__ __u32;

typedef __INT64_TYPE__ __i64;
typedef __UINT64_TYPE__ __u64;

typedef __INTPTR_TYPE__ __iptr;
typedef __UINTPTR_TYPE__ __uptr;

typedef __SIZE_TYPE__ __usize;

#endif
