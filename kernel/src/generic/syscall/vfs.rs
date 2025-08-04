use crate::generic::{
    memory::{VirtAddr, user::UserSlice},
    posix::errno::{EResult, Errno},
    sched::Scheduler,
    vfs::{
        File,
        file::{OpenFlags, SeekAnchor},
        inode::Mode,
    },
};
use alloc::borrow::ToOwned;
use core::ffi::{CStr, c_char};

pub fn read(fd: usize, addr: VirtAddr, len: usize) -> EResult<isize> {
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();

    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
    let slice = UserSlice::new(addr, len)
        .as_mut_slice(&proc_inner.address_space)
        .ok_or(Errno::EINVAL)?;
    drop(proc_inner);

    file.read(slice)
}

pub fn write(fd: usize, addr: VirtAddr, len: usize) -> EResult<isize> {
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();

    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
    let slice = UserSlice::new(addr, len)
        .as_slice(&proc_inner.address_space)
        .ok_or(Errno::EINVAL)?;
    drop(proc_inner);

    file.write(slice)
}

pub fn openat(fd: usize, path: usize, oflag: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let parent = if fd == uapi::AT_FDCWD as _ {
        None
    } else {
        Some(proc.inner.lock().get_fd(fd).ok_or(Errno::EBADF)?)
    };

    let path = unsafe { CStr::from_ptr(path as *const c_char) };
    let v = path.to_owned();
    let mut proc_inner = proc.inner.lock();

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

pub fn ioctl(fd: usize, request: usize, arg: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();
    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
    drop(proc_inner);

    file.ioctl(request, arg)
}
