use crate::generic::{
    clock,
    memory::user::UserPtr,
    posix::{
        errno::{EResult, Errno},
        utsname::UTSNAME,
    },
    sched::Scheduler,
};

pub fn archctl(cmd: usize, arg: usize) -> EResult<usize> {
    crate::arch::core::archctl(cmd, arg)
}

pub fn getuname(addr: UserPtr<uapi::utsname>) -> EResult<usize> {
    addr.write(UTSNAME.lock().clone()).ok_or(Errno::EINVAL)?;

    Ok(0)
}

pub fn setuname(addr: UserPtr<uapi::utsname>) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let inner = proc.inner.lock();
    if inner.identity.user_id != 0 {
        return Err(Errno::EPERM);
    }

    let mut utsname = UTSNAME.lock();
    *utsname = addr.read().ok_or(Errno::EINVAL)?;

    Ok(0)
}

pub fn clock_get(clockid: uapi::clockid_t, tp: UserPtr<uapi::timespec>) -> EResult<usize> {
    let _ = clockid; // TODO: Respect clockid

    let elapsed = clock::get_elapsed();

    tp.write(uapi::timespec {
        tv_sec: (elapsed / 1000 / 1000 / 1000) as _,
        tv_nsec: elapsed as _,
    })
    .ok_or(Errno::EINVAL)?;

    Ok(0)
}
