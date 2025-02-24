/* A global list of all system calls available. */

#ifndef SYSCALL
#define SYSCALL(num, name, ...)
#endif

SYSCALL(0, exit)
SYSCALL(1, uname)
SYSCALL(2, savetls)
SYSCALL(3, archctl)
SYSCALL(4, shutdown)
SYSCALL(5, reboot)
SYSCALL(6, readtimer)
SYSCALL(7, sigprocmask)
SYSCALL(8, sigsuspend)
SYSCALL(9, sigpending)
SYSCALL(0, sigaction)
SYSCALL(11, sigtimedwait)
SYSCALL(12, mmap)
SYSCALL(13, munmap)
SYSCALL(14, mprotect)
SYSCALL(15, execve)
SYSCALL(16, fork)
SYSCALL(17, kill)
SYSCALL(18, gettid)
SYSCALL(19, getpid)
SYSCALL(20, getppid)
SYSCALL(21, waitid)
SYSCALL(22, read)
SYSCALL(23, write)
SYSCALL(24, seek)
SYSCALL(25, ioctl)
SYSCALL(26, openat)
SYSCALL(27, close)
SYSCALL(28, stat)
SYSCALL(29, fstat)
SYSCALL(30, faccessat)
SYSCALL(31, fcntl)
SYSCALL(32, readdir)
SYSCALL(33, getcwd)
SYSCALL(34, chdir)
SYSCALL(35, fchdir)
SYSCALL(36, mkdirat)
SYSCALL(37, rmdirat)
SYSCALL(38, rename)
SYSCALL(39, chmodat)
SYSCALL(40, chownat)
SYSCALL(41, linkat)
SYSCALL(42, unlinkat)
SYSCALL(43, readlinkat)
SYSCALL(44, setuid)
SYSCALL(45, getuid)
SYSCALL(46, setgid)
SYSCALL(47, getgid)
SYSCALL(48, geteuid)
SYSCALL(49, getegid)
SYSCALL(50, getpgid)
SYSCALL(51, setpgid)
SYSCALL(52, umask)
SYSCALL(53, poll)
SYSCALL(54, pipe)
SYSCALL(55, chroot)
SYSCALL(56, mount)
SYSCALL(57, unmount)
SYSCALL(58, swapon)
SYSCALL(59, swapoff)
SYSCALL(60, futex_wait)
SYSCALL(61, futex_wake)
SYSCALL(62, socket)
SYSCALL(63, socketpair)
SYSCALL(64, bind)
SYSCALL(65, connect)
SYSCALL(66, accept)
SYSCALL(67, listen)
SYSCALL(68, getpeername)
SYSCALL(69, getsockname)
SYSCALL(70, getsockopt)
SYSCALL(71, setsockopt)
SYSCALL(72, recvmsg)
SYSCALL(73, sendmsg)
SYSCALL(74, sethostname)
SYSCALL(75, sched_setaffinity)
SYSCALL(76, sched_getaffinity)
