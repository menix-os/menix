use crate::{
    memory::{
        VirtAddr,
        user::{UserPtr, UserSlice},
    },
    posix::errno::{EResult, Errno},
    sched::Scheduler,
    vfs::{
        self, File, PathNode,
        cache::LookupFlags,
        file::{FileDescription, OpenFlags, SeekAnchor},
        inode::{INode, Mode, NodeOps, NodeType},
    },
};
use alloc::{borrow::ToOwned, sync::Arc};
use core::{
    ffi::CStr,
    sync::atomic::{AtomicBool, Ordering},
};

pub fn read(fd: usize, addr: VirtAddr, len: usize) -> EResult<isize> {
    let slice = UserSlice::new(addr, len)
        .as_mut_slice()
        .ok_or(Errno::EINVAL)?;
    let file = {
        let proc = Scheduler::get_current().get_process();
        let proc_inner = proc.inner.lock();
        proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file
    };

    let flags = *file.flags.lock();
    if !flags.contains(OpenFlags::Read) {
        return Err(Errno::EBADF);
    }

    file.read(slice)
}

pub fn pread(fd: usize, addr: VirtAddr, len: usize, offset: usize) -> EResult<isize> {
    let slice = UserSlice::new(addr, len)
        .as_mut_slice()
        .ok_or(Errno::EINVAL)?;
    let file = {
        let proc = Scheduler::get_current().get_process();
        let proc_inner = proc.inner.lock();
        proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file
    };

    let flags = *file.flags.lock();
    if !flags.contains(OpenFlags::Read) {
        return Err(Errno::EBADF);
    }

    file.pread(slice, offset as _)
}

pub fn write(fd: usize, addr: VirtAddr, len: usize) -> EResult<isize> {
    let slice = UserSlice::new(addr, len).as_slice().ok_or(Errno::EINVAL)?;
    let file = {
        let proc = Scheduler::get_current().get_process();
        let proc_inner = proc.inner.lock();
        proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file
    };

    let flags = *file.flags.lock();
    if !flags.contains(OpenFlags::Write) {
        return Err(Errno::EBADF);
    }
    file.write(slice)
}

pub fn pwrite(fd: usize, addr: VirtAddr, len: usize, offset: usize) -> EResult<isize> {
    let slice = UserSlice::new(addr, len).as_slice().ok_or(Errno::EINVAL)?;
    let file = {
        let proc = Scheduler::get_current().get_process();
        let proc_inner = proc.inner.lock();
        proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file
    };

    let flags = *file.flags.lock();
    if !flags.contains(OpenFlags::Write) {
        return Err(Errno::EBADF);
    }

    file.pwrite(slice, offset as _)
}

pub fn openat(fd: usize, path: VirtAddr, oflag: usize /* mode */) -> EResult<usize> {
    let path = unsafe { CStr::from_ptr(path.as_ptr()) };
    let v = path.to_owned();

    let proc = Scheduler::get_current().get_process();
    let mut proc_inner = proc.inner.lock();
    let parent = if fd == uapi::AT_FDCWD as _ {
        None
    } else {
        Some(proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file)
    };

    let file = File::open(
        &proc_inner,
        parent,
        v.to_bytes(),
        // O_CLOEXEC doesn't apply to a file, but rather its individual FD.
        // This means that dup'ing a file doesn't share this flag.
        OpenFlags::from_bits_truncate(oflag as _) & !OpenFlags::CloseOnExec,
        Mode::empty(),
        &proc_inner.identity,
    )?;

    proc_inner
        .open_file(
            FileDescription {
                file,
                close_on_exec: AtomicBool::new(
                    OpenFlags::from_bits_truncate(oflag as _).contains(OpenFlags::CloseOnExec),
                ),
            },
            0,
        )
        .ok_or(Errno::EMFILE)
}

pub fn seek(fd: usize, offset: usize, whence: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();
    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file;
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
        Some((_, desc)) => {
            // If this is the last reference to the underlying file, close it for good.
            if Arc::strong_count(&desc.file) == 1 {
                desc.file.close()?;
            }
            Ok(0)
        }
        // If it wasn't, the FD argument is invalid.
        None => Err(Errno::EBADF),
    }
}

pub fn ioctl(fd: usize, request: usize, arg: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();
    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file;
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

    // Walk up until we reach the root
    while let Ok(parent) = current.lookup_parent() {
        let name = &current.entry.name;
        if !name.is_empty() {
            // Copy name
            let len = name.len();
            cursor -= len;
            buffer[cursor..cursor + len].copy_from_slice(name);

            // Prepend slash
            cursor -= 1;
            buffer[cursor] = b'/';
        }
        current = parent;
    }

    // Special case: root directory
    if cursor == uapi::PATH_MAX as usize {
        cursor -= 1;
        buffer[cursor] = b'/';
    }

    let path_len = buffer.len() - cursor;
    if path_len + 1 > buf.len() {
        return Err(Errno::ERANGE);
    }

    buf[0..path_len].copy_from_slice(&buffer[cursor..]);
    buf[path_len] = 0; // NUL terminator

    Ok(path_len)
}

fn write_stat(inode: &Arc<INode>, statbuf: UserPtr<uapi::stat>) {
    let stat = uapi::stat {
        st_dev: 0,
        st_ino: inode.id,
        st_mode: inode.mode.load(Ordering::Acquire)
            | match inode.node_ops {
                NodeOps::Regular(_) => uapi::S_IFREG,
                NodeOps::Directory(_) => uapi::S_IFDIR,
                NodeOps::SymbolicLink(_) => uapi::S_IFLNK,
                NodeOps::FIFO => uapi::S_IFIFO,
                NodeOps::BlockDevice(_) => uapi::S_IFBLK,
                NodeOps::CharacterDevice(_) => uapi::S_IFCHR,
                NodeOps::Socket => uapi::S_IFSOCK,
            },
        st_nlink: Arc::strong_count(inode) as _,
        st_uid: inode.uid.load(Ordering::Acquire) as _,
        st_gid: inode.gid.load(Ordering::Acquire) as _,
        st_rdev: 0,
        st_size: inode.size.load(Ordering::Acquire) as _,
        st_atim: *inode.atime.lock(),
        st_mtim: *inode.mtime.lock(),
        st_ctim: *inode.ctime.lock(),
        st_blksize: 0,
        st_blocks: 0,
    };

    statbuf.write(stat);
}

pub fn fstat(fd: usize, statbuf: UserPtr<uapi::stat>) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();

    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file;
    let inode = file.inode.as_ref().ok_or(Errno::EINVAL)?;

    write_stat(inode, statbuf);

    Ok(0)
}

pub fn fstatat(
    at: usize,
    path: VirtAddr,
    statbuf: UserPtr<uapi::stat>,
    _flags: usize, // TODO
) -> EResult<usize> {
    let path = unsafe { CStr::from_ptr(path.as_ptr()) };
    let v = path.to_owned();

    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.inner.lock();
    let parent = if at == uapi::AT_FDCWD as _ {
        None
    } else {
        Some(proc_inner.get_fd(at).ok_or(Errno::EBADF)?.file)
    };

    let file = File::open(
        &proc_inner,
        parent,
        v.to_bytes(),
        OpenFlags::Read,
        Mode::empty(),
        &proc_inner.identity,
    )?;
    let inode = file.inode.as_ref().ok_or(Errno::EINVAL)?;

    drop(proc_inner);

    write_stat(inode, statbuf);

    Ok(0)
}

pub fn dup(fd: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let mut proc_inner = proc.inner.lock();
    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
    proc_inner.open_file(file, fd).ok_or(Errno::EMFILE)
}

pub fn dup3(fd1: usize, fd2: usize, _flags: usize) -> EResult<usize> {
    if fd1 == fd2 {
        return Ok(fd1);
    }

    let proc = Scheduler::get_current().get_process();
    let mut proc_inner = proc.inner.lock();

    let file = proc_inner.get_fd(fd1).ok_or(Errno::EBADF)?;
    proc_inner.open_files.insert(fd2, file);
    Ok(fd2)
}

pub fn mkdirat(fd: usize, path: VirtAddr, mode: uapi::mode_t) -> EResult<usize> {
    let path = unsafe { CStr::from_ptr(path.as_ptr()) };
    let v = path.to_owned();

    let proc = Scheduler::get_current().get_process();
    let inner = proc.inner.lock();
    let parent = if fd == uapi::AT_FDCWD as _ {
        None
    } else {
        Some(inner.get_fd(fd).ok_or(Errno::EBADF)?.file)
    };
    vfs::mknod(
        &inner,
        parent,
        v.as_bytes(),
        NodeType::Directory,
        Mode::from_bits(mode).ok_or(Errno::EINVAL)?,
        None,
        &inner.identity,
    )?;

    Ok(0)
}

pub fn chdir(path: VirtAddr) -> EResult<usize> {
    let path = unsafe { CStr::from_ptr(path.as_ptr()) };
    let v = path.to_owned();

    let proc = Scheduler::get_current().get_process();
    let mut inner = proc.inner.lock();
    let node = PathNode::lookup(
        &inner,
        None,
        v.as_bytes(),
        &inner.identity,
        LookupFlags::MustExist,
    )?;
    inner.working_dir = node;

    Ok(0)
}

pub fn getdents(fd: usize, addr: VirtAddr, len: usize) -> EResult<usize> {
    let buf: &mut [u8] = UserSlice::new(addr, len)
        .as_mut_slice()
        .ok_or(Errno::EINVAL)?;

    let proc = Scheduler::get_current().get_process();
    let inner = proc.inner.lock();

    // fd must be a valid descriptor open for reading.
    let dir = inner.get_fd(fd).ok_or(Errno::EBADF)?.file;
    let flags = *dir.flags.lock();
    if !flags.contains(OpenFlags::Read | OpenFlags::Directory) {
        return Err(Errno::EBADF);
    }

    // fd must be a directory.
    let node = dir.inode.clone().ok_or(Errno::EBADF)?;
    match &node.node_ops {
        NodeOps::Directory(dir_ops) => {
            // TODO: VFS Probably need a getdents callback...
            _ = (buf, dir_ops);
        }
        _ => return Err(Errno::ENOTDIR),
    }

    Ok(0) // TODO
}

pub fn fcntl(fd: usize, cmd: usize, arg: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let mut proc_inner = proc.inner.lock();

    match cmd as _ {
        uapi::F_DUPFD => {
            let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
            proc_inner.open_file(file, arg).ok_or(Errno::EMFILE)
        }
        uapi::F_DUPFD_CLOEXEC => {
            let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
            file.close_on_exec.store(true, Ordering::Release);
            proc_inner.open_file(file, arg).ok_or(Errno::EMFILE)
        }
        uapi::F_GETFD => {
            let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
            let mut flags = OpenFlags::empty();
            flags.set(
                OpenFlags::CloseOnExec,
                file.close_on_exec.load(Ordering::Acquire),
            );
            Ok(flags.bits() as _)
        }
        uapi::F_SETFD => Ok(0),
        uapi::F_GETFL => {
            let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
            let flags = *file.file.flags.lock();
            Ok(flags.bits() as _)
        }
        uapi::F_SETFL => {
            warn!("fcntl F_SETFL is a stub!");
            Ok(0)
        }
        uapi::F_GETOWN => {
            warn!("fcntl F_GETOWN is a stub!");
            Ok(0)
        }
        uapi::F_SETOWN => {
            warn!("fcntl F_SETOWN is a stub!");
            Ok(0)
        }
        uapi::F_GETOWN_EX => {
            warn!("fcntl F_GETOWN_EX is a stub!");
            Ok(0)
        }
        uapi::F_SETOWN_EX => {
            warn!("fcntl F_SETOWN_EX is a stub!");
            Ok(0)
        }
        uapi::F_GETLK => {
            warn!("fcntl F_GETLK is a stub!");
            Ok(0)
        }
        uapi::F_SETLK => {
            warn!("fcntl F_SETLK is a stub!");
            Ok(0)
        }
        uapi::F_SETLKW => {
            warn!("fcntl F_SETLKW is a stub!");
            Ok(0)
        }
        uapi::F_OFD_GETLK => {
            warn!("fcntl F_OFD_GETLK is a stub!");
            Ok(0)
        }
        uapi::F_OFD_SETLK => {
            warn!("fcntl F_OFD_SETLK is a stub!");
            Ok(0)
        }
        uapi::F_OFD_SETLKW => {
            warn!("fcntl F_OFD_SETLKW is a stub!");
            Ok(0)
        }
        _ => Err(Errno::EINVAL),
    }
}

pub fn pselect(
    fd: usize,
    read_fds: VirtAddr,
    write_fds: VirtAddr,
    except_fds: VirtAddr,
    timeout: UserPtr<uapi::timespec>,
    sigmask: UserPtr<uapi::sigset_t>,
) -> EResult<usize> {
    let _ = (except_fds, sigmask, timeout, write_fds, read_fds, fd);
    // TODO
    Ok(1)
}

pub fn pipe(filedes: UserPtr<[i32; 2]>) -> EResult<usize> {
    let fds = {
        let proc = Scheduler::get_current().get_process();
        let mut proc_inner = proc.inner.lock();
        let (pipe1, pipe2) = vfs::pipe()?;
        [
            proc_inner
                .open_file(
                    FileDescription {
                        file: pipe1,
                        close_on_exec: AtomicBool::new(false),
                    },
                    0,
                )
                .ok_or(Errno::EMFILE)? as _,
            proc_inner
                .open_file(
                    FileDescription {
                        file: pipe2,
                        close_on_exec: AtomicBool::new(false),
                    },
                    0,
                )
                .ok_or(Errno::EMFILE)? as _,
        ]
    };

    filedes.write(fds);
    Ok(0)
}
