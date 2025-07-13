use crate::generic::{
    posix::errno::{EResult, Errno},
    sched::Scheduler,
    vfs::{File, file::OpenFlags, inode::Mode},
};
use core::ffi::{CStr, c_char};

pub fn read(fd: usize, buf: usize, len: usize) -> EResult<isize> {
    // TODO: Use User{Ptr,Slice} instead of raw pointers.
    let buf: &mut [u8] = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, len) };

    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();
    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;

    file.read(buf)
}

pub fn write(fd: usize, buf: usize, len: usize) -> EResult<isize> {
    // TODO: Use User{Ptr,Slice} instead of raw pointers.
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };

    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();
    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;

    file.write(buf)
}

pub fn openat(fd: usize, path: usize, oflag: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let mut proc_inner = proc.inner.lock();

    let parent = if fd == uapi::AT_FDCWD as _ {
        None
    } else {
        Some(proc_inner.get_fd(fd).ok_or(Errno::EBADF)?)
    };

    // TODO: Use UserCStr
    let path = unsafe { CStr::from_ptr(path as *const c_char) };
    let file = File::open(
        &proc_inner,
        parent,
        path.to_bytes(),
        OpenFlags::from_bits_truncate(oflag as _),
        Mode::empty(),
        &proc_inner.identity,
    )?;

    proc_inner.add_file(file).ok_or(Errno::EMFILE)
}
