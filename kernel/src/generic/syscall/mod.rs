mod numbers;
mod process;

use super::posix::errno::{EResult, Errno};
use alloc::string::String;

/// Executes the syscall as identified by `num`.
/// Returns a tuple of (value, error) to the user. An error code of 0 inidcates success.
/// If the error code is not 0, `value` is not valid and indicates failure.
pub fn dispatch(
    num: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> (usize, usize) {
    warn!("running syscall {num} with args {a0:x}, {a1:x}");
    let result = match DISPATCH_TABLE.get(num).and_then(|&x| x) {
        Some(x) => x(a0, a1, a2, a3, a4, a5),
        None => {
            warn!("Unknown syscall {num}");
            Err(Errno::ENOSYS)
        }
    };

    match result {
        Ok(x) => return (x, 0),
        Err(x) => return (0, x as usize),
    }
}

type SyscallHandler = fn(usize, usize, usize, usize, usize, usize) -> EResult<usize>;

static DISPATCH_TABLE: [Option<SyscallHandler>; 135] = {
    let mut table = [None as Option<SyscallHandler>; _];
    table[numbers::EXIT] = Some(process::exit);
    table[numbers::SYSLOG] = None;
    table[numbers::UNAME] = Some(process::uname);
    table[numbers::ARCHCTL] = Some(crate::arch::core::archctl);
    table[numbers::REBOOT] = None;
    table[numbers::MMAP] = None;
    table[numbers::MUNMAP] = None;
    table[numbers::MPROTECT] = None;
    table[numbers::MADVISE] = None;
    table[numbers::SIGPROCMASK] = None;
    table[numbers::SIGSUSPEND] = None;
    table[numbers::SIGPENDING] = None;
    table[numbers::SIGACTION] = None;
    table[numbers::SIGTIMEDWAIT] = None;
    table[numbers::SIGALTSTACK] = None;
    table[numbers::EXECVE] = None;
    table[numbers::FORK] = None;
    table[numbers::KILL] = None;
    table[numbers::GETTID] = Some(process::gettid);
    table[numbers::GETPID] = None;
    table[numbers::GETPPID] = None;
    table[numbers::WAITID] = None;
    table[numbers::WAITPID] = None;
    table[numbers::READ] = None;
    table[numbers::PREAD] = None;
    table[numbers::WRITE] = None;
    table[numbers::PWRITE] = None;
    table[numbers::SEEK] = None;
    table[numbers::IOCTL] = None;
    table[numbers::OPENAT] = None;
    table[numbers::CLOSE] = None;
    table[numbers::STAT] = None;
    table[numbers::FSTAT] = None;
    table[numbers::STATVFS] = None;
    table[numbers::FSTATVFS] = None;
    table[numbers::FACCESSAT] = None;
    table[numbers::FCNTL] = None;
    table[numbers::FTRUNCATE] = None;
    table[numbers::FALLOCATE] = None;
    table[numbers::UTIMENSAT] = None;
    table[numbers::PSELECT] = None;
    table[numbers::MKNODAT] = None;
    table[numbers::READDIR] = None;
    table[numbers::GETCWD] = None;
    table[numbers::CHDIR] = None;
    table[numbers::FCHDIR] = None;
    table[numbers::MKDIRAT] = None;
    table[numbers::RMDIRAT] = None;
    table[numbers::GETDENTS] = None;
    table[numbers::RENAMEAT] = None;
    table[numbers::FCHMOD] = None;
    table[numbers::FCHMODAT] = None;
    table[numbers::FCHOWNAT] = None;
    table[numbers::LINKAT] = None;
    table[numbers::SYMLINKAT] = None;
    table[numbers::UNLINKAT] = None;
    table[numbers::READLINKAT] = None;
    table[numbers::FLOCK] = None;
    table[numbers::POLL] = None;
    table[numbers::DUP] = None;
    table[numbers::DUP3] = None;
    table[numbers::SYNC] = None;
    table[numbers::FSYNC] = None;
    table[numbers::FDATASYNC] = None;
    table[numbers::GETGROUPS] = None;
    table[numbers::SETGROUPS] = None;
    table[numbers::GETSID] = None;
    table[numbers::SETSID] = None;
    table[numbers::SETUID] = None;
    table[numbers::GETUID] = None;
    table[numbers::SETGID] = None;
    table[numbers::GETGID] = None;
    table[numbers::GETEUID] = None;
    table[numbers::SETEUID] = None;
    table[numbers::GETEGID] = None;
    table[numbers::SETEGID] = None;
    table[numbers::GETPGID] = None;
    table[numbers::SETPGID] = None;
    table[numbers::GETRESUID] = None;
    table[numbers::SETRESUID] = None;
    table[numbers::GETRESGID] = None;
    table[numbers::SETRESGID] = None;
    table[numbers::SETREUID] = None;
    table[numbers::SETREGID] = None;
    table[numbers::UMASK] = None;
    table[numbers::PIPE] = None;
    table[numbers::FUTEX_WAIT] = None;
    table[numbers::FUTEX_WAKE] = None;
    table[numbers::THREAD_CREATE] = None;
    table[numbers::THREAD_KILL] = None;
    table[numbers::THREAD_EXIT] = None;
    table[numbers::THREAD_SETNAME] = None;
    table[numbers::THREAD_GETNAME] = None;
    table[numbers::TIMER_CREATE] = None;
    table[numbers::TIMER_SET] = None;
    table[numbers::TIMER_DELETE] = None;
    table[numbers::ITIMER_GET] = None;
    table[numbers::ITIMER_SET] = None;
    table[numbers::CLOCK_GET] = None;
    table[numbers::CLOCK_GETRES] = None;
    table[numbers::SLEEP] = None;
    table[numbers::YIELD] = None;
    table[numbers::CHROOT] = None;
    table[numbers::MOUNT] = None;
    table[numbers::UMOUNT] = None;
    table[numbers::SWAPON] = None;
    table[numbers::SWAPOFF] = None;
    table[numbers::SOCKET] = None;
    table[numbers::SOCKETPAIR] = None;
    table[numbers::SHUTDOWN] = None;
    table[numbers::BIND] = None;
    table[numbers::CONNECT] = None;
    table[numbers::ACCEPT] = None;
    table[numbers::LISTEN] = None;
    table[numbers::GETPEERNAME] = None;
    table[numbers::GETSOCKNAME] = None;
    table[numbers::GETSOCKOPT] = None;
    table[numbers::SETSOCKOPT] = None;
    table[numbers::SENDMSG] = None;
    table[numbers::SENDTO] = None;
    table[numbers::RECVMSG] = None;
    table[numbers::RECVFROM] = None;
    table[numbers::GETHOSTNAME] = None;
    table[numbers::SETHOSTNAME] = None;
    table[numbers::GETENTROPY] = None;
    table[numbers::GETRUSAGE] = None;
    table[numbers::GETRLIMIT] = None;
    table[numbers::SETRLIMIT] = None;
    table[numbers::GETPRIORITY] = None;
    table[numbers::SETPRIORITY] = None;
    table[numbers::SCHED_GETPARAM] = None;
    table[numbers::SCHED_SETPARAM] = None;
    table[numbers::GETCPU] = None;
    table[numbers::SYSINFO] = None;
    table[numbers::PTRACE] = None;

    // TODO: TTY stuff
    table[numbers::WRITE] = Some(|fd, buf, len, _, _, _| {
        let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };
        use core::fmt::Write;
        {
            let mut writer = crate::generic::log::GLOBAL_LOGGERS.lock();
            _ = writer.write_str(&String::from_utf8_lossy(buf));
        }
        Ok(len)
    });

    table
};
