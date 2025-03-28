use crate::generic::log::GLOBAL_LOGGERS;
use core::{fmt::Write, ptr::slice_from_raw_parts};

use super::errno::Errno;

const EXIT: usize = 0;
const UNAME: usize = 1;
const SAVE_TLS: usize = 2;
const ARCHCTL: usize = 3;
const SHUTDOWN: usize = 4;
const REBOOT: usize = 5;
const READ_TIMER: usize = 6;
const SIG_PROCMASK: usize = 7;
const SIG_SUSPEND: usize = 8;
const SIG_PENDING: usize = 9;
const SIG_ACTION: usize = 10;
const SIG_TIMEDWAIT: usize = 11;
const MMAP: usize = 12;
const MUNMAP: usize = 13;
const MPROTECT: usize = 14;
const EXECVE: usize = 15;
const FORK: usize = 16;
const KILL: usize = 17;
const GETTID: usize = 18;
const GETPID: usize = 19;
const GETPPID: usize = 20;
const WAITID: usize = 21;
const READ: usize = 22;
const WRITE: usize = 23;
const SEEK: usize = 24;
const IOCTL: usize = 25;
const OPENAT: usize = 26;
const CLOSE: usize = 27;
const STAT: usize = 28;
const FSTAT: usize = 29;
const FACCESSAT: usize = 30;
const FCNTL: usize = 31;
const READDIR: usize = 32;
const GETCWD: usize = 33;
const CHDIR: usize = 34;
const FCHDIR: usize = 35;
const MKDIRAT: usize = 36;
const RMDIRAT: usize = 37;
const RENAME: usize = 38;
const CHMODAT: usize = 39;
const CHOWNAT: usize = 40;
const LINKAT: usize = 41;
const UNLINKAT: usize = 42;
const READLINKAT: usize = 43;
const SETUID: usize = 44;
const GETUID: usize = 45;
const SETGID: usize = 46;
const GETGID: usize = 47;
const GETEUID: usize = 48;
const GETEGID: usize = 49;
const GETPGID: usize = 50;
const SETPGID: usize = 51;
const UMASK: usize = 52;
const POLL: usize = 53;
const PIPE: usize = 54;
const CHROOT: usize = 55;
const MOUNT: usize = 56;
const UNMOUNT: usize = 57;
const SWAPON: usize = 58;
const SWAPOFF: usize = 59;
const FUTEX_WAIT: usize = 60;
const FUTEX_WAKE: usize = 61;
const SOCKET: usize = 62;
const SOCKET_PAIR: usize = 63;
const BIND: usize = 64;
const CONNECT: usize = 65;
const ACCEPT: usize = 66;
const LISTEN: usize = 67;
const GETPEERNAME: usize = 68;
const GETSOCKNAME: usize = 69;
const GETSOCKOPT: usize = 70;
const SETSOCKOPT: usize = 71;
const RECVMSG: usize = 72;
const SENDMSG: usize = 73;
const SETHOSTNAME: usize = 74;
const SCHED_SET_AFFINITY: usize = 75;
const SCHED_GET_AFFINITY: usize = 76;

/// Executes the syscall as identified by `num`.
pub fn invoke(
    num: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> (usize, usize) {
    let result: Result<usize, Errno> = match num {
        EXIT => todo!(),
        UNAME => todo!(),
        SAVE_TLS => todo!(),
        ARCHCTL => todo!(),
        SHUTDOWN => todo!(),
        REBOOT => todo!(),
        READ_TIMER => todo!(),
        SIG_PROCMASK => todo!(),
        SIG_SUSPEND => todo!(),
        SIG_PENDING => todo!(),
        SIG_ACTION => todo!(),
        SIG_TIMEDWAIT => todo!(),
        MMAP => todo!(),
        MUNMAP => todo!(),
        MPROTECT => todo!(),
        EXECVE => todo!(),
        FORK => todo!(),
        KILL => todo!(),
        GETTID => todo!(),
        GETPID => todo!(),
        GETPPID => todo!(),
        WAITID => todo!(),
        READ => todo!(),
        WRITE => todo!(),
        SEEK => todo!(),
        IOCTL => todo!(),
        OPENAT => todo!(),
        CLOSE => todo!(),
        STAT => todo!(),
        FSTAT => todo!(),
        FACCESSAT => todo!(),
        FCNTL => todo!(),
        READDIR => todo!(),
        GETCWD => todo!(),
        CHDIR => todo!(),
        FCHDIR => todo!(),
        MKDIRAT => todo!(),
        RMDIRAT => todo!(),
        RENAME => todo!(),
        CHMODAT => todo!(),
        CHOWNAT => todo!(),
        LINKAT => todo!(),
        UNLINKAT => todo!(),
        READLINKAT => todo!(),
        SETUID => todo!(),
        GETUID => todo!(),
        SETGID => todo!(),
        GETGID => todo!(),
        GETEUID => todo!(),
        GETEGID => todo!(),
        GETPGID => todo!(),
        SETPGID => todo!(),
        UMASK => todo!(),
        POLL => todo!(),
        PIPE => todo!(),
        CHROOT => todo!(),
        MOUNT => todo!(),
        UNMOUNT => todo!(),
        SWAPON => todo!(),
        SWAPOFF => todo!(),
        FUTEX_WAIT => todo!(),
        FUTEX_WAKE => todo!(),
        SOCKET => todo!(),
        SOCKET_PAIR => todo!(),
        BIND => todo!(),
        CONNECT => todo!(),
        ACCEPT => todo!(),
        LISTEN => todo!(),
        GETPEERNAME => todo!(),
        GETSOCKNAME => todo!(),
        GETSOCKOPT => todo!(),
        SETSOCKOPT => todo!(),
        RECVMSG => todo!(),
        SENDMSG => todo!(),
        SETHOSTNAME => todo!(),
        SCHED_SET_AFFINITY => todo!(),
        SCHED_GET_AFFINITY => todo!(),
        _ => {
            print!(
                "syscall: Unknown syscall {:#x} requested by user program\n",
                num
            );
            Err(Errno::ENOSYS)
        }
    };

    match result {
        Ok(x) => return (x as usize, 0),
        Err(x) => return (0, x as usize),
    }
}
