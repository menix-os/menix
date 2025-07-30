#ifndef _UAPI_MENIX_FCNTL_H
#define _UAPI_MENIX_FCNTL_H

#include <uapi/menix/types.h>
#include <uapi/menix/posix_types.h>

#define __O_PATH    010000000
#define __O_ACCMODE (03 | __O_PATH)
#define __O_RDONLY  00
#define __O_WRONLY  01
#define __O_RDWR    02

#define __O_CREAT     0100
#define __O_EXCL      0200
#define __O_NOCTTY    0400
#define __O_TRUNC     01000
#define __O_APPEND    02000
#define __O_NONBLOCK  04000
#define __O_DSYNC     010000
#define __O_ASYNC     020000
#define __O_DIRECT    040000
#define __O_DIRECTORY 0200000
#define __O_NOFOLLOW  0400000
#define __O_CLOEXEC   02000000
#define __O_SYNC      04010000
#define __O_RSYNC     04010000
#define __O_LARGEFILE 0100000
#define __O_NOATIME   01000000
#define __O_TMPFILE   020000000

#define __O_EXEC   __O_PATH
#define __O_SEARCH __O_PATH

#define __F_DUPFD 0
#define __F_GETFD 1
#define __F_SETFD 2
#define __F_GETFL 3
#define __F_SETFL 4

#define __F_SETOWN 8
#define __F_GETOWN 9
#define __F_SETSIG 10
#define __F_GETSIG 11

#define __F_GETLK    5
#define __F_SETLK    6
#define __F_SETLK64  __F_SETLK
#define __F_SETLKW   7
#define __F_SETLKW64 __F_SETLKW

#define __F_SETOWN_EX 15
#define __F_GETOWN_EX 16

#define __F_GETOWNER_UIDS 17

#define __F_SETLEASE      1024
#define __F_GETLEASE      1025
#define __F_NOTIFY        1026
#define __F_DUPFD_CLOEXEC 1030
#define __F_SETPIPE_SZ    1031
#define __F_GETPIPE_SZ    1032
#define __F_ADD_SEALS     1033
#define __F_GET_SEALS     1034

#define __F_SEAL_SEAL   0x0001
#define __F_SEAL_SHRINK 0x0002
#define __F_SEAL_GROW   0x0004
#define __F_SEAL_WRITE  0x0008

#define __F_OFD_GETLK  36
#define __F_OFD_SETLK  37
#define __F_OFD_SETLKW 38

#define __F_RDLCK 0
#define __F_WRLCK 1
#define __F_UNLCK 2

#define __FD_CLOEXEC 1

#define __AT_FDCWD            -100
#define __AT_SYMLINK_NOFOLLOW 0x100
#define __AT_REMOVEDIR        0x200
#define __AT_SYMLINK_FOLLOW   0x400
#define __AT_EACCESS          0x200
#define __AT_NO_AUTOMOUNT     0x800
#define __AT_EMPTY_PATH       0x1000

#define __AT_STATX_SYNC_AS_STAT 0x0000
#define __AT_STATX_FORCE_SYNC   0x2000
#define __AT_STATX_DONT_SYNC    0x4000
#define __AT_STATX_SYNC_TYPE    0x6000

struct __f_owner_ex {
    __i32 type;
    __pid_t pid;
};

#define __F_OWNER_TID 0

#define __POSIX_FADV_NORMAL     0
#define __POSIX_FADV_RANDOM     1
#define __POSIX_FADV_SEQUENTIAL 2
#define __POSIX_FADV_WILLNEED   3
#define __POSIX_FADV_DONTNEED   4
#define __POSIX_FADV_NOREUSE    5

#endif
