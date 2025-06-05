#ifndef __MENIX_UAPI_FCNTL_H
#define __MENIX_UAPI_FCNTL_H

#define O_ACCMODE (3 << 0)
#define O_RDONLY (1 << 0)
#define O_WRONLY (1 << 1)
#define O_RDWR (3 << 0)
#define O_APPEND (1 << 2)
#define O_CLOEXEC (1 << 3)
#define O_NOFOLLOW (1 << 4)
#define O_NONBLOCK (1 << 5)

#define O_CREAT (1 << 7)
#define O_EXCL (1 << 8)
#define O_TRUNC (1 << 9)

#define O_SEARCH (1 << 10)
#define O_EXEC (1 << 11)
#define O_NOCTTY (1 << 12)
#define O_DSYNC (1 << 13)
#define O_RSYNC (1 << 14)
#define O_SYNC (1 << 15)
#define O_PATH (1 << 16)
#define O_DIRECTORY (1 << 17)
#define O_LARGEFILE (1 << 18)
#define O_NOATIME (1 << 19)
#define O_TMPFILE (1 << 20)

#define FD_CLOEXEC 1
#define F_DUPFD 1
#define F_DUPFD_CLOEXEC 2
#define F_GETFD 3
#define F_SETFD 4
#define F_GETFL 5
#define F_SETFL 6
#define F_GETPIPE_SZ 7
#define F_SETPIPE_SZ 8
#define F_GETLK 9
#define F_SETLK 10
#define F_SETLKW 11

#define F_RDLCK 1
#define F_UNLCK 2
#define F_WRLCK 3

#define F_GETOWN 10
#define F_SETOWN 11

#define F_SEAL_SHRINK 0x0002
#define F_SEAL_GROW 0x0004
#define F_SEAL_WRITE 0x0008
#define F_SEAL_SEAL 0x0010
#define F_ADD_SEALS 1033
#define F_GET_SEALS 1034

#define AT_REMOVEDIR 0x200
#define AT_EACCESS 0x200
#define AT_FDCWD 0x7FFFFFFF
#define AT_EMPTY_PATH 0x1000
#define AT_SYMLINK_FOLLOW 0x400
#define AT_SYMLINK_NOFOLLOW 0x100

#define AT_STATX_SYNC_AS_STAT 0x0000
#define AT_STATX_FORCE_SYNC 0x2000
#define AT_STATX_DONT_SYNC 0x4000
#define AT_STATX_SYNC_TYPE 0x6000

#define POSIX_FADV_NORMAL 1
#define POSIX_FADV_SEQUENTIAL 2
#define POSIX_FADV_NOREUSE 3
#define POSIX_FADV_DONTNEED 4
#define POSIX_FADV_WILLNEED 5
#define POSIX_FADV_RANDOM 6

#endif
