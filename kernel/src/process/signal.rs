use uapi::signal::*;

#[repr(u32)]
pub enum Signal {
    SIGHUP = SIGHUP,
    SIGINT = SIGINT,
    SIGQUIT = SIGQUIT,
    SIGCONT = SIGCONT,
    SIGBUS = SIGBUS,
    SIGABRT = SIGABRT,
    SIGCHLD = SIGCHLD,
    SIGFPE = SIGFPE,
    SIGKILL = SIGKILL,
    SIGILL = SIGILL,
    SIGPIPE = SIGPIPE,
    SIGSEGV = SIGSEGV,
    SIGSTOP = SIGSTOP,
    SIGALRM = SIGALRM,
    SIGTERM = SIGTERM,
    SIGTSTP = SIGTSTP,
    SIGTTIN = SIGTTIN,
    SIGTTOU = SIGTTOU,
    SIGUSR1 = SIGUSR1,
    SIGUSR2 = SIGUSR2,
    SIGIO = SIGIO,
    SIGPROF = SIGPROF,
    SIGSYS = SIGSYS,
    SIGTRAP = SIGTRAP,
    SIGURG = SIGURG,
    SIGVTALRM = SIGVTALRM,
    SIGXCPU = SIGXCPU,
    SIGXFSZ = SIGXFSZ,
    SIGWINCH = SIGWINCH,
    SIGPWR = SIGPWR,
}

/// Wrapper around a set of signals.
#[repr(transparent)]
pub struct SignalSet {
    inner: sigset_t,
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
