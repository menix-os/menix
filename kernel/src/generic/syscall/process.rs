use crate::generic::{
    memory::user::UserPtr,
    posix::{self, errno::EResult},
    sched::Scheduler,
};
use menix_proc::syscall;

#[syscall]
pub fn gettid() -> EResult<usize> {
    Ok(Scheduler::get_current().get_id())
}

#[syscall]
pub fn getpid() -> EResult<usize> {
    Ok(Scheduler::get_current().get_process().get_pid())
}

#[syscall]
pub fn exit(error: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();

    if proc.get_pid() <= 1 {
        panic!("Attempted to kill init with error code {error}");
    }

    todo!()
}

#[syscall]
pub fn uname(addr: UserPtr<uapi::utsname>) -> EResult<usize> {
    let mut utsname = uapi::utsname::default();
    posix::utsname::utsname(&mut utsname);
    addr.write(utsname);
    Ok(0)
}
