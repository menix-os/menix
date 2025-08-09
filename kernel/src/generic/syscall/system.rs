use crate::generic::{
    memory::user::UserPtr,
    posix::{
        self,
        errno::{EResult, Errno},
    },
};

pub fn archctl(cmd: usize, arg: usize) -> EResult<usize> {
    crate::arch::core::archctl(cmd, arg)
}

pub fn uname(addr: UserPtr<uapi::utsname>) -> EResult<usize> {
    let mut utsname = uapi::utsname::default();

    posix::utsname::utsname(&mut utsname);
    addr.write(utsname).ok_or(Errno::EINVAL)?;

    Ok(0)
}
