#ifndef __MENIX_UAPI_STATVFS_H
#define __MENIX_UAPI_STATVFS_H

#include "types.h"

#define ST_RDONLY 1
#define ST_NOSUID 2
#define ST_NODEV 4
#define ST_NOEXEC 8
#define ST_SYNCHRONOUS 16
#define ST_MANDLOCK 64
#define ST_WRITE 128
#define ST_APPEND 256
#define ST_IMMUTABLE 512
#define ST_NOATIME 1024
#define ST_NODIRATIME 2048

struct statvfs {
  __usize f_bsize;
  __usize f_frsize;
  fsblkcnt_t f_blocks;
  fsblkcnt_t f_bfree;
  fsblkcnt_t f_bavail;
  fsfilcnt_t f_files;
  fsfilcnt_t f_ffree;
  fsfilcnt_t f_favail;
  __usize f_fsid;
  __usize f_flag;
  __usize f_namemax;
};

#endif
