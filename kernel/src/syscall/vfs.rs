use crate::uapi::{fcntl::*, limits::PATH_MAX, mode_t, signal::sigset_t, stat::*, time::timespec};
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
        inode::{INode, Mode, NodeOps},
    },
};
use alloc::{borrow::ToOwned, sync::Arc};
use core::{
    ffi::CStr,
    sync::atomic::{AtomicBool, Ordering},
};

pub fn read(fd: usize, addr: VirtAddr, len: usize) -> EResult<isize> {
    let mut user_ptr = UserSlice::new(addr, len);
    let slice = user_ptr.as_mut_slice().ok_or(Errno::EINVAL)?;
    let file = {
        let proc = Scheduler::get_current().get_process();
        let proc_inner = proc.open_files.lock();
        proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file
    };

    let flags = *file.flags.lock();
    if !flags.contains(OpenFlags::Read) {
        return Err(Errno::EBADF);
    }

    file.read(slice)
}

pub fn pread(fd: usize, addr: VirtAddr, len: usize, offset: usize) -> EResult<isize> {
    let mut user_ptr = UserSlice::new(addr, len);
    let slice = user_ptr.as_mut_slice().ok_or(Errno::EINVAL)?;
    let file = {
        let proc = Scheduler::get_current().get_process();
        let proc_inner = proc.open_files.lock();
        proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file
    };

    let flags = *file.flags.lock();
    if !flags.contains(OpenFlags::Read) {
        return Err(Errno::EBADF);
    }

    file.pread(slice, offset as _)
}

pub fn write(fd: usize, addr: VirtAddr, len: usize) -> EResult<isize> {
    let user_ptr = UserSlice::new(addr, len);
    let slice = user_ptr.as_slice().ok_or(Errno::EINVAL)?;
    let file = {
        let proc = Scheduler::get_current().get_process();
        let proc_inner = proc.open_files.lock();
        proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file
    };

    let flags = *file.flags.lock();
    if !flags.contains(OpenFlags::Write) {
        return Err(Errno::EBADF);
    }
    file.write(slice)
}

pub fn pwrite(fd: usize, addr: VirtAddr, len: usize, offset: usize) -> EResult<isize> {
    let user_ptr = UserSlice::new(addr, len);
    let slice = user_ptr.as_slice().ok_or(Errno::EINVAL)?;
    let file = {
        let proc = Scheduler::get_current().get_process();
        let proc_inner = proc.open_files.lock();
        proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file
    };

    let flags = *file.flags.lock();
    if !flags.contains(OpenFlags::Write) {
        return Err(Errno::EBADF);
    }

    file.pwrite(slice, offset as _)
}

pub fn openat(fd: usize, path: VirtAddr, oflag: usize /* mode */) -> EResult<usize> {
    // TODO: This should really be using UserPtr/a CStr abstraction.
    if path == VirtAddr::null() {
        return Err(Errno::EINVAL);
    }

    let path = unsafe { CStr::from_ptr(path.as_ptr()) };
    let v = path.to_owned();

    let proc = Scheduler::get_current().get_process();
    let mut proc_inner = proc.open_files.lock();
    let parent = if fd == AT_FDCWD as _ {
        proc.working_dir.lock().clone()
    } else {
        proc_inner
            .get_fd(fd)
            .ok_or(Errno::EBADF)?
            .file
            .path
            .as_ref()
            .ok_or(Errno::ENOTDIR)?
            .clone()
    };

    let file = File::open(
        proc.root_dir.lock().clone(),
        parent,
        v.to_bytes(),
        // O_CLOEXEC doesn't apply to a file, but rather its individual FD.
        // This means that dup'ing a file doesn't share this flag.
        OpenFlags::from_bits_truncate(oflag as _) & !OpenFlags::CloseOnExec,
        Mode::empty(),
        &proc.identity.lock(),
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
    let file = proc.open_files.lock().get_fd(fd).ok_or(Errno::EBADF)?.file;
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
    let mut proc_inner = proc.open_files.lock();

    proc_inner.close(fd).ok_or(Errno::EBADF)?;
    Ok(0)
}

pub fn ioctl(fd: usize, request: usize, arg: VirtAddr) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.open_files.lock();
    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file;
    drop(proc_inner);

    file.ioctl(request, arg)
}

pub fn getcwd(buffer: VirtAddr, len: usize) -> EResult<usize> {
    let mut user_ptr = UserSlice::new(buffer, len);
    let buf: &mut [u8] = user_ptr.as_mut_slice().ok_or(Errno::EINVAL)?;

    let proc = Scheduler::get_current().get_process();

    let mut buffer = vec![0u8; PATH_MAX as _];
    let mut cursor = PATH_MAX as usize;
    let mut current = proc.working_dir.lock().clone();

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
    if cursor == PATH_MAX as usize {
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

fn write_stat(inode: &Arc<INode>, mut statbuf: UserPtr<stat>) {
    statbuf.write(stat {
        st_dev: 0,
        st_ino: inode.id,
        st_mode: inode.mode.lock().bits()
            | match inode.node_ops {
                NodeOps::Regular(_) => S_IFREG,
                NodeOps::Directory(_) => S_IFDIR,
                NodeOps::SymbolicLink(_) => S_IFLNK,
                NodeOps::FIFO => S_IFIFO,
                NodeOps::BlockDevice => S_IFBLK,
                NodeOps::CharacterDevice => S_IFCHR,
                NodeOps::Socket => S_IFSOCK,
            },
        st_nlink: Arc::strong_count(inode) as _,
        st_uid: *inode.uid.lock(),
        st_gid: *inode.gid.lock(),
        st_rdev: 0,
        st_size: *inode.size.lock() as _,
        st_atim: *inode.atime.lock(),
        st_mtim: *inode.mtime.lock(),
        st_ctim: *inode.ctime.lock(),
        st_blksize: 0,
        st_blocks: 0,
    });
}

pub fn fstat(fd: usize, statbuf: UserPtr<stat>) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.open_files.lock();

    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?.file;
    let inode = file.inode.as_ref().ok_or(Errno::EINVAL)?;

    write_stat(inode, statbuf);

    Ok(0)
}

pub fn fstatat(
    at: usize,
    path: VirtAddr,
    statbuf: UserPtr<stat>,
    _flags: usize, // TODO
) -> EResult<usize> {
    let path = unsafe { CStr::from_ptr(path.as_ptr()) };
    let v = path.to_owned();

    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.open_files.lock();
    let parent = if at == AT_FDCWD as _ {
        proc.working_dir.lock().clone()
    } else {
        proc_inner
            .get_fd(at)
            .ok_or(Errno::EBADF)?
            .file
            .path
            .as_ref()
            .ok_or(Errno::ENOTDIR)?
            .clone()
    };

    let file = File::open(
        proc.root_dir.lock().clone(),
        parent,
        v.to_bytes(),
        OpenFlags::Read,
        Mode::empty(),
        &proc.identity.lock(),
    )?;
    let inode = file.inode.as_ref().ok_or(Errno::EINVAL)?;

    drop(proc_inner);

    write_stat(inode, statbuf);

    Ok(0)
}

pub fn dup(fd: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let mut proc_inner = proc.open_files.lock();
    let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
    proc_inner.open_file(file, fd).ok_or(Errno::EMFILE)
}

pub fn dup3(fd1: usize, fd2: usize, flags: usize) -> EResult<usize> {
    if fd1 == fd2 {
        return Ok(fd1);
    }

    let proc = Scheduler::get_current().get_process();
    let mut proc_inner = proc.open_files.lock();

    let file = proc_inner.get_fd(fd1).ok_or(Errno::EBADF)?;
    if proc_inner.get_fd(fd2).is_some() {
        proc_inner.close(fd2);
    }

    let flags = OpenFlags::from_bits_truncate(flags as _);
    if flags.contains(OpenFlags::CloseOnExec) {
        file.close_on_exec.store(true, Ordering::Release);
    }

    proc_inner.open_file(file, fd2).ok_or(Errno::EMFILE)
}

pub fn mkdirat(fd: usize, path: VirtAddr, mode: mode_t) -> EResult<usize> {
    let path = unsafe { CStr::from_ptr(path.as_ptr()) };
    let v = path.to_owned();

    let proc = Scheduler::get_current().get_process();
    let inner = proc.open_files.lock();
    let parent = if fd == AT_FDCWD as _ {
        proc.working_dir.lock().clone()
    } else {
        inner
            .get_fd(fd)
            .ok_or(Errno::EBADF)?
            .file
            .path
            .as_ref()
            .ok_or(Errno::ENOTDIR)?
            .clone()
    };
    vfs::mkdir(
        proc.root_dir.lock().clone(),
        parent,
        v.as_bytes(),
        Mode::from_bits(mode).ok_or(Errno::EINVAL)?,
        &proc.identity.lock(),
    )?;

    Ok(0)
}

pub fn chdir(path: VirtAddr) -> EResult<usize> {
    let path = unsafe { CStr::from_ptr(path.as_ptr()) };
    let v = path.to_owned();

    let proc = Scheduler::get_current().get_process();
    let root = proc.root_dir.lock();
    let mut cwd = proc.working_dir.lock();
    let node = PathNode::lookup(
        root.clone(),
        cwd.clone(),
        v.as_bytes(),
        &proc.identity.lock(),
        LookupFlags::MustExist,
    )?;
    *cwd = node;

    Ok(0)
}

pub fn fchdir(fd: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();
    let mut cwd = proc.working_dir.lock();
    let dir = proc.open_files.lock().get_fd(fd).ok_or(Errno::EBADF)?;
    *cwd = dir.file.path.as_ref().cloned().ok_or(Errno::ENOTDIR)?;

    Ok(0)
}

pub fn getdents(fd: usize, addr: VirtAddr, len: usize) -> EResult<usize> {
    let mut user_ptr = UserSlice::new(addr, len);
    let buf: &mut [u8] = user_ptr.as_mut_slice().ok_or(Errno::EINVAL)?;

    let proc = Scheduler::get_current().get_process();
    let inner = proc.open_files.lock();

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
    let mut proc_inner = proc.open_files.lock();

    match cmd as _ {
        F_DUPFD => {
            let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
            proc_inner.open_file(file, arg).ok_or(Errno::EMFILE)
        }
        F_DUPFD_CLOEXEC => {
            let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
            file.close_on_exec.store(true, Ordering::Release);
            proc_inner.open_file(file, arg).ok_or(Errno::EMFILE)
        }
        F_GETFD => {
            let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
            let mut flags = OpenFlags::empty();
            flags.set(
                OpenFlags::CloseOnExec,
                file.close_on_exec.load(Ordering::Acquire),
            );
            Ok(flags.bits() as _)
        }
        F_SETFD => Ok(0),
        F_GETFL => {
            let file = proc_inner.get_fd(fd).ok_or(Errno::EBADF)?;
            let flags = *file.file.flags.lock();
            Ok(flags.bits() as _)
        }
        F_SETFL => {
            warn!("fcntl F_SETFL is a stub!");
            Ok(0)
        }
        F_GETOWN => {
            warn!("fcntl F_GETOWN is a stub!");
            Ok(0)
        }
        F_SETOWN => {
            warn!("fcntl F_SETOWN is a stub!");
            Ok(0)
        }
        F_GETOWN_EX => {
            warn!("fcntl F_GETOWN_EX is a stub!");
            Ok(0)
        }
        F_SETOWN_EX => {
            warn!("fcntl F_SETOWN_EX is a stub!");
            Ok(0)
        }
        F_GETLK => {
            warn!("fcntl F_GETLK is a stub!");
            Ok(0)
        }
        F_SETLK => {
            warn!("fcntl F_SETLK is a stub!");
            Ok(0)
        }
        F_SETLKW => {
            warn!("fcntl F_SETLKW is a stub!");
            Ok(0)
        }
        F_OFD_GETLK => {
            warn!("fcntl F_OFD_GETLK is a stub!");
            Ok(0)
        }
        F_OFD_SETLK => {
            warn!("fcntl F_OFD_SETLK is a stub!");
            Ok(0)
        }
        F_OFD_SETLKW => {
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
    timeout: UserPtr<timespec>,
    sigmask: UserPtr<sigset_t>,
) -> EResult<usize> {
    let _ = (except_fds, sigmask, timeout, write_fds, read_fds, fd);
    // TODO
    Ok(1)
}

pub fn pipe(mut filedes: UserPtr<[i32; 2]>) -> EResult<usize> {
    let fds = {
        let proc = Scheduler::get_current().get_process();
        let mut files = proc.open_files.lock();
        let (pipe1, pipe2) = vfs::pipe()?;
        [
            files
                .open_file(
                    FileDescription {
                        file: pipe1,
                        close_on_exec: AtomicBool::new(false),
                    },
                    0,
                )
                .ok_or(Errno::EMFILE)? as _,
            files
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

pub fn faccessat(fd: usize, path: VirtAddr, amode: usize, flag: usize) -> EResult<usize> {
    if path == VirtAddr::null() {
        return Err(Errno::EINVAL);
    }

    let path = unsafe { CStr::from_ptr(path.as_ptr()) };
    let v = path.to_owned();

    let proc = Scheduler::get_current().get_process();
    let proc_inner = proc.open_files.lock();
    let parent = if fd == AT_FDCWD as _ {
        proc.working_dir.lock().clone()
    } else {
        proc_inner
            .get_fd(fd)
            .ok_or(Errno::EBADF)?
            .file
            .path
            .as_ref()
            .ok_or(Errno::ENOTDIR)?
            .clone()
    };

    let path_node = PathNode::lookup(
        proc.root_dir.lock().clone(),
        parent,
        v.as_bytes(),
        &proc.identity.lock(),
        LookupFlags::MustExist
            | LookupFlags::FollowSymlinks
            | if flag as u32 & AT_EACCESS != 0 {
                LookupFlags::empty()
            } else {
                LookupFlags::UseRealId
            },
    )?;

    let node = path_node.entry.get_inode().ok_or(Errno::EBADF)?;
    let amode = Mode::from_bits_truncate(amode as _);
    if !node.mode.lock().intersects(amode) {
        return Err(Errno::EACCES);
    }

    Ok(0)
}
