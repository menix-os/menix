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
    let slice = UserSlice::new(addr, len)
        .as_mut_slice()
        .ok_or(Errno::EINVAL)?;
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();

    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
    drop(proc_inner);

    file.read(slice)
}

pub fn pread(fd: usize, addr: VirtAddr, len: usize, offset: usize) -> EResult<isize> {
    let slice = UserSlice::new(addr, len)
        .as_mut_slice()
        .ok_or(Errno::EINVAL)?;
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();

    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
    drop(proc_inner);

    file.pread(slice, offset as _)
}

pub fn write(fd: usize, addr: VirtAddr, len: usize) -> EResult<isize> {
    let slice = UserSlice::new(addr, len).as_slice().ok_or(Errno::EINVAL)?;
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();

    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
    drop(proc_inner);

    file.write(slice)
}

pub fn pwrite(fd: usize, addr: VirtAddr, len: usize, offset: usize) -> EResult<isize> {
    let slice = UserSlice::new(addr, len).as_slice().ok_or(Errno::EINVAL)?;
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();

    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
    drop(proc_inner);

    file.pwrite(slice, offset as _)
}

pub fn openat(fd: usize, path: usize, oflag: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let parent = if fd == uapi::AT_FDCWD as _ {
        None
    } else {
        Some(proc.inner.lock().get_fd(fd).ok_or(Errno::EBADF)?)
    };

    // TODO: Use UserPtr instead.
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

pub fn getcwd(buffer: VirtAddr, len: usize) -> EResult<usize> {
    let buf: &mut [u8] = UserSlice::new(buffer, len)
        .as_mut_slice()
        .ok_or(Errno::EINVAL)?;

    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();

    let mut buffer = vec![0u8; uapi::PATH_MAX as _];
    let mut cursor = uapi::PATH_MAX as usize;
    let mut current = proc_inner.working_dir.clone();
    while cursor > 0 {
        let len = current.entry.name.len();
        // Write the component to the buffer.
        cursor -= len;
        buffer[cursor..][..len].copy_from_slice(&current.entry.name);

        // Write the path separator.
        cursor -= 1;
        buffer[cursor] = b'/';

        let Ok(res) = current.lookup_parent() else {
            break;
        };
        current = res;
    }
    buf[0..buffer.len() - cursor].copy_from_slice(&buffer[cursor..]);

    Ok(0)
}
