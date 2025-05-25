#ifndef __MENIX_UAPI_TYPES_H
#define __MENIX_UAPI_TYPES_H

typedef signed char __i8;
typedef unsigned char __u8;

typedef signed short __i16;
typedef unsigned short __u16;

typedef signed int __i32;
typedef unsigned int __u32;

typedef signed long __i64;
typedef unsigned long __u64;

typedef __SIZE_TYPE__ size_t;

typedef long off_t;
typedef long off64_t;

typedef long blksize_t;
typedef long blkcnt_t;
typedef long clockid_t;
typedef unsigned long dev_t;
typedef unsigned int gid_t;
typedef long ino_t;
typedef int mode_t;
typedef int nlink_t;
typedef int pid_t;
typedef unsigned long long rlim_t;
typedef unsigned int uid_t;
typedef __u64 fsblkcnt_t;
typedef __u64 fsfilcnt_t;

#endif
