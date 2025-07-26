use alloc::{borrow::ToOwned, string::String};

use crate::generic::{
    log::GLOBAL_LOGGERS,
    posix::errno::{EResult, Errno},
    sched::Scheduler,
    vfs::{
        File,
        file::{OpenFlags, SeekAnchor},
        inode::Mode,
    },
};
use core::{
    ffi::{CStr, c_char},
    fmt::Write,
};

pub fn read(fd: usize, buf: usize, len: usize) -> EResult<isize> {
    // TODO: Use User{Ptr,Slice} instead of raw pointers.
    let buf = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, len) };

    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();
    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
    drop(proc_inner);

    file.read(buf)
}

pub fn write(fd: usize, buf: usize, len: usize) -> EResult<isize> {
    // TODO: Use User{Ptr,Slice} instead of raw pointers.
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };

    if fd == 1 || fd == 2 {
        let mut log = GLOBAL_LOGGERS.lock();
        log.write_str(&String::from_utf8_lossy(buf)).unwrap();
        return Ok(len as _);
    }

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
    let v = path.to_owned();
    let file = File::open(
        &proc_inner,
        parent,
        v.to_bytes(),
        OpenFlags::from_bits_truncate(oflag as _),
        Mode::empty(),
        &proc_inner.identity,
    )?;

    proc_inner.open_file(file).ok_or(Errno::EMFILE)
}

pub fn seek(fd: usize, offset: usize, whence: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();
    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
    let anchor = match whence {
        0 => SeekAnchor::Start(offset as _),
        1 => SeekAnchor::Current(offset as _),
        2 => SeekAnchor::End(offset as _),
        _ => return Err(Errno::EINVAL),
    };
    file.seek(anchor).map(|x| x as _)
}

pub fn close(fd: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let mut proc_inner = proc.inner.lock();

    match proc_inner.open_files.remove_entry(&fd) {
        // If removal was successful, the close worked.
        Some(_) => Ok(0),
        // If it wasn't, the FD argument is invalid.
        None => Err(Errno::EBADF),
    }
}
