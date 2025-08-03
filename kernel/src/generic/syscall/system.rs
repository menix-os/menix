use crate::generic::{
    memory::user::UserPtr,
    posix::{
        self,
        errno::{EResult, Errno},
    },
    sched::Scheduler,
};

pub fn archctl(cmd: usize, arg: usize) -> EResult<usize> {
    crate::arch::core::archctl(cmd, arg)
}

pub fn uname(addr: UserPtr<uapi::utsname>) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();
    let mut utsname = uapi::utsname::default();

    posix::utsname::utsname(&mut utsname);
    addr.write(&proc_inner.address_space, utsname)
        .ok_or(Errno::EINVAL)?;

    Ok(0)
}
