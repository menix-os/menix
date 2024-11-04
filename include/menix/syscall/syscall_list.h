// A global list of all system calls available.

#ifdef SYSCALL_TABLE_INSERT
#define SYSCALL(num, name) [num] = (SyscallFn)syscall_##name,
#else
#include <menix/common.h>
typedef struct SyscallResult SyscallResult;
#define SYSCALL(num, name) SyscallResult syscall_##name(usize a0, usize a1, usize a2, usize a3, usize a4, usize a5);
#endif

SYSCALL(0, exit)
SYSCALL(1, open)
SYSCALL(2, close)
SYSCALL(3, stat)
SYSCALL(4, access)
SYSCALL(5, read)
SYSCALL(6, write)
SYSCALL(7, seek)
SYSCALL(8, ioctl)
SYSCALL(9, chdir)
SYSCALL(10, chmod)
SYSCALL(11, chown)
SYSCALL(12, mount)
SYSCALL(13, unmount)
SYSCALL(14, link)
SYSCALL(15, unlink)
SYSCALL(16, symlink)
SYSCALL(17, readlink)
SYSCALL(18, rmdir)
SYSCALL(19, setuid)
SYSCALL(20, getuid)
SYSCALL(21, setgid)
SYSCALL(22, getgid)
SYSCALL(23, sync)
SYSCALL(24, mmap)
SYSCALL(25, munmap)
SYSCALL(26, mremap)
SYSCALL(27, mprotect)
SYSCALL(28, execve)
SYSCALL(29, fork)
SYSCALL(30, kill)
SYSCALL(31, getpid)
SYSCALL(32, getparentpid)
SYSCALL(33, recvmsg)
SYSCALL(34, sendmsg)
SYSCALL(35, accept)
SYSCALL(36, getpeername)
SYSCALL(37, getsockname)
SYSCALL(38, uname)
SYSCALL(39, powerctl)
SYSCALL(40, archctl)
SYSCALL(41, getcwd)
