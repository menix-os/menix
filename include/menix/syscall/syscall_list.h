// A global list of all system calls available.

#ifdef SYSCALL_TABLE_INSERT
#define SYSCALL(num, name) [num] = {.func = (SyscallFn)syscall_##name, .func_name = #name},
#else
#include <menix/common.h>
typedef struct SyscallResult SyscallResult;
#define SYSCALL(num, name) SyscallResult syscall_##name(usize a0, usize a1, usize a2, usize a3, usize a4, usize a5);
#endif

SYSCALL(0, exit)
// Signals
SYSCALL(1, sigprocmask)
SYSCALL(2, sigsuspend)
SYSCALL(3, sigpending)
SYSCALL(4, sigaction)
SYSCALL(5, sigreturn)
SYSCALL(6, sigtimedwait)
// Memory
SYSCALL(7, mmap)
SYSCALL(8, munmap)
SYSCALL(9, mremap)
SYSCALL(10, mprotect)
// Threads
SYSCALL(11, execve)
SYSCALL(12, fork)
SYSCALL(13, kill)
SYSCALL(14, getpid)
SYSCALL(15, waitpid)
// VFS
SYSCALL(16, read)
SYSCALL(17, write)
SYSCALL(18, seek)
SYSCALL(19, ioctl)
SYSCALL(20, openat)
SYSCALL(21, close)
SYSCALL(22, stat)
SYSCALL(23, faccessat)
SYSCALL(24, fcntl)
SYSCALL(25, readdir)
SYSCALL(26, getcwd)
SYSCALL(27, chdir)
SYSCALL(28, fchdir)
SYSCALL(29, mkdirat)
SYSCALL(30, rename)
SYSCALL(31, chmodat)
SYSCALL(32, chownat)
SYSCALL(33, linkat)
SYSCALL(34, unlinkat)
SYSCALL(35, readlinkat)
SYSCALL(36, mount)
SYSCALL(37, unmount)
SYSCALL(38, setuid)
SYSCALL(39, getuid)
SYSCALL(30, setgid)
SYSCALL(40, getgid)
SYSCALL(41, umask)
SYSCALL(42, poll)
SYSCALL(43, isatty)
SYSCALL(44, chroot)
// Futex
SYSCALL(45, futex_wait)
SYSCALL(46, futex_wake)
// Sockets
SYSCALL(47, socket)
SYSCALL(48, socketpair)
SYSCALL(49, bind)
SYSCALL(50, connect)
SYSCALL(51, accept)
SYSCALL(52, listen)
SYSCALL(53, getpeername)
SYSCALL(54, getsockname)
SYSCALL(55, getsockopt)
SYSCALL(56, setsockopt)
SYSCALL(57, recvmsg)
SYSCALL(58, sendmsg)
SYSCALL(59, sethostname)
// Misc
SYSCALL(60, uname)
SYSCALL(61, archctl)
