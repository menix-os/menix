#ifndef _UAPI_MENIX_MOUNT_H
#define _UAPI_MENIX_MOUNT_H

#define __MS_RDONLY       1
#define __MS_NOSUID       2
#define __MS_NODEV        4
#define __MS_NOEXEC       8
#define __MS_SYNCHRONOUS  16
#define __MS_REMOUNT      32
#define __MS_MANDLOCK     64
#define __MS_DIRSYNC      128
#define __MS_NOSYMFOLLOW  256
#define __MS_NOATIME      1024
#define __MS_NODIRATIME   2048
#define __MS_BIND         4096
#define __MS_MOVE         8192
#define __MS_REC          16384
#define __MS_SILENT       32768
#define __MS_POSIXACL     (1 << 16)
#define __MS_UNBINDABLE   (1 << 17)
#define __MS_PRIVATE      (1 << 18)
#define __MS_SLAVE        (1 << 19)
#define __MS_SHARED       (1 << 20)
#define __MS_RELATIME     (1 << 21)
#define __MS_KERNMOUNT    (1 << 22)
#define __MS_I_VERSION    (1 << 23)
#define __MS_STRICTATIME  (1 << 24)
#define __MS_LAZYTIME     (1 << 25)
#define __MS_NOREMOTELOCK (1 << 27)
#define __MS_NOSEC        (1 << 28)
#define __MS_BORN         (1 << 29)
#define __MS_ACTIVE       (1 << 30)
#define __MS_NOUSER       (1 << 31)

#endif
