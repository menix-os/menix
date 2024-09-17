// ABI for working with the POSIX/C interface.
// Definitions are copied from mlibc.

#pragma once
#include <menix/common.h>

typedef isize ssize_t;
typedef isize off_t;
typedef usize dev_t;
typedef usize ino_t;
typedef u32 mode_t;
typedef usize nlink_t;
typedef isize blksize_t;
typedef isize blkcnt_t;
typedef i32 pid_t;
typedef i32 uid_t;
typedef i32 gid_t;
typedef isize time_t;
typedef isize clockid_t;
typedef u32 socklen_t;

struct timespec
{
	time_t tv_sec;
	isize tv_nsec;
};

#define IOV_MAX	 1024
#define OPEN_MAX 256
#define NAME_MAX 255
#define PATH_MAX 4096

struct stat
{
	dev_t st_dev;
	ino_t st_ino;
	nlink_t st_nlink;
	mode_t st_mode;
	uid_t st_uid;
	gid_t st_gid;
	u32 __pad0;
	dev_t st_rdev;
	off_t st_size;
	blksize_t st_blksize;
	blkcnt_t st_blocks;
	struct timespec st_atim;
	struct timespec st_mtim;
	struct timespec st_ctim;
	u32 __unused[6];
};

struct utsname
{
	char sysname[65];
	char nodename[65];
	char release[65];
	char version[65];
	char machine[65];
};

#define O_PATH 010000000

#define O_ACCMODE (03 | O_PATH)
#define O_RDONLY  00
#define O_WRONLY  01
#define O_RDWR	  02

#define O_CREAT		0100
#define O_EXCL		0200
#define O_NOCTTY	0400
#define O_TRUNC		01000
#define O_APPEND	02000
#define O_NONBLOCK	04000
#define O_DSYNC		010000
#define O_ASYNC		020000
#define O_DIRECT	040000
#define O_DIRECTORY 0200000
#define O_NOFOLLOW	0400000
#define O_CLOEXEC	02000000
#define O_SYNC		04010000
#define O_RSYNC		04010000
#define O_LARGEFILE 0100000
#define O_NOATIME	01000000
#define O_TMPFILE	020000000

#define O_EXEC	 O_PATH
#define O_SEARCH O_PATH

#define F_DUPFD 0
#define F_GETFD 1
#define F_SETFD 2
#define F_GETFL 3
#define F_SETFL 4

#define F_SETOWN 8
#define F_GETOWN 9
#define F_SETSIG 10
#define F_GETSIG 11

#define F_GETLK	 5
#define F_SETLK	 6
#define F_SETLKW 7

#define F_SETOWN_EX 15
#define F_GETOWN_EX 16

#define F_GETOWNER_UIDS 17

#define F_DUPFD_CLOEXEC 1030
#define F_ADD_SEALS		1033
#define F_GET_SEALS		1034

#define F_SEAL_SEAL	  0x0001
#define F_SEAL_SHRINK 0x0002
#define F_SEAL_GROW	  0x0004
#define F_SEAL_WRITE  0x0008

#define F_RDLCK 0
#define F_WRLCK 1
#define F_UNLCK 2

#define FD_CLOEXEC 1

#define AT_FDCWD			-100
#define AT_SYMLINK_NOFOLLOW 0x100
#define AT_REMOVEDIR		0x200
#define AT_SYMLINK_FOLLOW	0x400
#define AT_EACCESS			0x200
#define AT_EMPTY_PATH		0x1000

#define S_IFMT	 0x0F000
#define S_IFBLK	 0x06000
#define S_IFCHR	 0x02000
#define S_IFIFO	 0x01000
#define S_IFREG	 0x08000
#define S_IFDIR	 0x04000
#define S_IFLNK	 0x0A000
#define S_IFSOCK 0x0C000

#define S_IRWXU 0700
#define S_IRUSR 0400
#define S_IWUSR 0200
#define S_IXUSR 0100
#define S_IRWXG 070
#define S_IRGRP 040
#define S_IWGRP 020
#define S_IXGRP 010
#define S_IRWXO 07
#define S_IROTH 04
#define S_IWOTH 02
#define S_IXOTH 01
#define S_ISUID 04000
#define S_ISGID 02000
#define S_ISVTX 01000

#define S_IREAD	 S_IRUSR
#define S_IWRITE S_IWUSR
#define S_IEXEC	 S_IXUSR

#define S_ISBLK(m)	(((m) & S_IFMT) == S_IFBLK)
#define S_ISCHR(m)	(((m) & S_IFMT) == S_IFCHR)
#define S_ISFIFO(m) (((m) & S_IFMT) == S_IFIFO)
#define S_ISREG(m)	(((m) & S_IFMT) == S_IFREG)
#define S_ISDIR(m)	(((m) & S_IFMT) == S_IFDIR)
#define S_ISLNK(m)	(((m) & S_IFMT) == S_IFLNK)
#define S_ISSOCK(m) (((m) & S_IFMT) == S_IFSOCK)

#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

#define TCGETS	   0x5401
#define TCSETS	   0x5402
#define TCSETSW	   0x5403
#define TCSETSF	   0x5404
#define TCSBRK	   0x5409
#define TCXONC	   0x540A
#define TIOCSCTTY  0x540E
#define TIOCSTI	   0x5412
#define TIOCGWINSZ 0x5413
#define TIOCMGET   0x5415
#define TIOCMSET   0x5418
#define TIOCINQ	   0x541B
#define TIOCNOTTY  0x5422

#define POLLIN	   0x01
#define POLLOUT	   0x02
#define POLLPRI	   0x04
#define POLLHUP	   0x08
#define POLLERR	   0x10
#define POLLRDHUP  0x20
#define POLLNVAL   0x40
#define POLLWRNORM 0x80
#define POLLRDNORM 0x100
#define POLLWRBAND 0x200
#define POLLRDBAND 0x400

#define SOCK_CLOEXEC  O_CLOEXEC
#define SOCK_NONBLOCK O_NONBLOCK

#define PROT_NONE  0x00
#define PROT_READ  0x01
#define PROT_WRITE 0x02
#define PROT_EXEC  0x04

#define MAP_FAILED			((void*)(-1))
#define MAP_FILE			0x00
#define MAP_PRIVATE			0x01
#define MAP_SHARED			0x02
#define MAP_FIXED			0x04
#define MAP_ANON			0x08
#define MAP_ANONYMOUS		0x08
#define MAP_NORESERVE		0x10
#define MAP_FIXED_NOREPLACE 0x20

#define MS_ASYNC	  0x01
#define MS_SYNC		  0x02
#define MS_INVALIDATE 0x04

#define MCL_CURRENT 0x01
#define MCL_FUTURE	0x02

#define POSIX_MADV_NORMAL	  1
#define POSIX_MADV_SEQUENTIAL 2
#define POSIX_MADV_RANDOM	  3
#define POSIX_MADV_DONTNEED	  4
#define POSIX_MADV_WILLNEED	  5

#define MADV_NORMAL		0
#define MADV_RANDOM		1
#define MADV_SEQUENTIAL 2
#define MADV_WILLNEED	3
#define MADV_DONTNEED	4
#define MADV_FREE		8
