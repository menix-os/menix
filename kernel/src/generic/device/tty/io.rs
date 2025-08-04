use crate::generic::{
    memory::user::UserPtr,
    posix::errno::{EResult, Errno},
    sched::Scheduler,
};

impl super::Tty {
    pub fn tiocgwinsz(&self, arg: UserPtr<uapi::winsize>) -> EResult<()> {
        let proc = Scheduler::get_current().get_process();
        let proc_inner = proc.inner.lock();
        arg.write(&proc_inner.address_space, *self.winsize.lock())
            .ok_or(Errno::EINVAL)
    }
}
