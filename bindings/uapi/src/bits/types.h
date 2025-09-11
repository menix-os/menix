#ifndef __MENIX_UAPI_TYPES_H
#define __MENIX_UAPI_TYPES_H

typedef signed char __i8;
typedef unsigned char __u8;
typedef short __i16;
typedef unsigned short __u16;
typedef int __i32;
typedef unsigned int __u32;
typedef long __i64;
typedef unsigned long __u64;
typedef long __isize;
typedef __SIZE_TYPE__ __usize;

typedef long off_t;
typedef long off64_t;

typedef __usize blksize_t;
typedef __usize blkcnt_t;
typedef __usize clockid_t;
typedef __usize dev_t;
typedef __usize gid_t;
typedef __usize ino_t;
typedef __u32 mode_t;
typedef __usize nlink_t;
typedef __usize pid_t;
typedef __usize rlim_t;
typedef __usize uid_t;
typedef __usize fsblkcnt_t;
typedef __usize fsfilcnt_t;
typedef __usize pthread_attr_t;

#endif
