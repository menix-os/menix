#[repr(u32)]
pub enum Signal {
    SIGHUP = uapi::SIGHUP,
    SIGINT = uapi::SIGINT,
    SIGQUIT = uapi::SIGQUIT,
    SIGCONT = uapi::SIGCONT,
    SIGBUS = uapi::SIGBUS,
    SIGABRT = uapi::SIGABRT,
    SIGCHLD = uapi::SIGCHLD,
    SIGFPE = uapi::SIGFPE,
    SIGKILL = uapi::SIGKILL,
    SIGILL = uapi::SIGILL,
    SIGPIPE = uapi::SIGPIPE,
    SIGSEGV = uapi::SIGSEGV,
    SIGSTOP = uapi::SIGSTOP,
    SIGALRM = uapi::SIGALRM,
    SIGTERM = uapi::SIGTERM,
    SIGTSTP = uapi::SIGTSTP,
    SIGTTIN = uapi::SIGTTIN,
    SIGTTOU = uapi::SIGTTOU,
    SIGUSR1 = uapi::SIGUSR1,
    SIGUSR2 = uapi::SIGUSR2,
    SIGIO = uapi::SIGIO,
    SIGPROF = uapi::SIGPROF,
    SIGSYS = uapi::SIGSYS,
    SIGTRAP = uapi::SIGTRAP,
    SIGURG = uapi::SIGURG,
    SIGVTALRM = uapi::SIGVTALRM,
    SIGXCPU = uapi::SIGXCPU,
    SIGXFSZ = uapi::SIGXFSZ,
    SIGWINCH = uapi::SIGWINCH,
    SIGPWR = uapi::SIGPWR,
}

/// Wrapper around a set of signals.
#[repr(transparent)]
pub struct SignalSet {
    inner: uapi::sigset_t,
}

impl SignalSet {
    pub fn new() -> Self {
        Self { inner: 0 }
    }

    pub fn set_signal(&mut self, idx: usize, state: bool) {
        if state {
            self.inner |= 1 << idx;
        } else {
            self.inner &= !(1 << idx);
        }
    }
}
