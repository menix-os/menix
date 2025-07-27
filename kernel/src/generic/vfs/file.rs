use super::inode::{INode, NodeType};
use crate::generic::{
    memory::cache::MemoryObject,
    posix::errno::{EResult, Errno},
    process::{Identity, InnerProcess},
    vfs::{
        cache::{LookupFlags, PathNode},
        inode::{Mode, NodeOps},
    },
};
use alloc::sync::Arc;
use core::{
    fmt::Debug,
    num::NonZeroUsize,
    sync::atomic::{AtomicUsize, Ordering},
};

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct OpenFlags: u32 {
        /// Create the file if it's missing.
        const Create = uapi::O_CREAT as _;
        /// Exclusive use.
        const Exclusive = uapi::O_EXCL as _;
        /// Do not assign a controlling terminal.
        const NoCtrlTerminal = uapi::O_NOCTTY as _;
        const Truncate = uapi::O_TRUNC as _;
        const Append = uapi::O_APPEND as _;
        const NonBlocking = uapi::O_NONBLOCK as _;
        const SyncData = uapi::O_DSYNC as _;
        /// Open this file as a directory.
        const Directory = uapi::O_DIRECTORY as _;
        /// Don't follow symbolic links.
        const NoFollow = uapi::O_NOFOLLOW as _;
        /// Close this file on a call to `execve`.
        const CloseOnExec = uapi::O_CLOEXEC as _;
        const Sync = uapi::O_SYNC as _;
        const SyncRead = uapi::O_RSYNC as _;
        const LargeFile = uapi::O_LARGEFILE as _;
        /// Don't update the access time.
        const NoAccessTime = uapi::O_NOATIME as _;
        const Temporary = uapi::O_TMPFILE as _;
        const ReadOnly = uapi::O_RDONLY as _;
        const WriteOnly = uapi::O_WRONLY as _;
        const ReadWrite = uapi::O_RDWR as _;
        const Executable = uapi::O_EXEC as _;
    }

    #[derive(Debug, Clone, Copy)]
    pub struct MmapFlags: u32 {
        const Anonymous = uapi::MAP_ANONYMOUS as _;
        const Shared = uapi::MAP_SHARED as _;
        const Private = uapi::MAP_PRIVATE as _;
        const Fixed = uapi::MAP_FIXED as _;
    }
}

pub enum SeekAnchor {
    /// Seek relative to the start of the file.
    Start(u64),
    /// Seek relative to the current cursor position.
    Current(i64),
    /// Seek relative to the end of the file.
    End(i64),
}

/// The kernel representation of an open file.
#[derive(Debug)]
pub struct File {
    /// The cached entry for this file.
    pub path: Option<PathNode>,
    /// Operations that can be performed on this file.
    pub ops: Arc<dyn FileOps>,
    /// The opened inode.
    pub inode: Option<Arc<INode>>,
    /// File open flags.
    pub flags: OpenFlags,
    /// The cursor of this file.
    pub position: AtomicUsize,
}

/// Operations that can be performed on a file. Every trait function has a
/// generic implementation, which treats it as unimplemented.
/// Inputs have been sanitized when these functions are called.
pub trait FileOps: Debug {
    /// Reads from the file into a buffer.
    /// Returns actual bytes read and the new offset.
    fn read(&self, file: &File, buffer: &mut [u8], offset: uapi::off_t) -> EResult<isize> {
        let _ = (offset, buffer, file);
        Ok(0)
    }

    /// Writes a buffer to the file.
    /// Returns actual bytes written.
    fn write(&self, file: &File, buffer: &[u8], offset: uapi::off_t) -> EResult<isize> {
        let _ = (offset, buffer, file);
        Ok(0)
    }

    /// Performs a generic ioctl operation on the file.
    /// Returns a driver specific status code if it was successful.
    fn ioctl(&self, file: &File, request: usize, arg: usize) -> EResult<usize> {
        _ = (arg, request, file);
        Err(Errno::ENOTTY)
    }

    /// Polls this file with a mask.
    fn poll(&self, file: &File, mask: u16) -> EResult<u16> {
        _ = (file, mask);
        Ok(mask)
    }

    fn seek(&self, file: &File, offset: SeekAnchor) -> EResult<uapi::off_t>;
}

impl File {
    /// Opens a file referenced by a path.
    pub fn open(
        proc: &InnerProcess,
        at: Option<Arc<File>>,
        path: &[u8],
        flags: OpenFlags,
        mode: Mode,
        identity: &Identity,
    ) -> EResult<Arc<Self>> {
        if flags.contains(OpenFlags::Directory)
            && flags.intersects(OpenFlags::Create | OpenFlags::Temporary)
        {
            return Err(Errno::EINVAL);
        }

        let mut lookup_flags = LookupFlags::empty();
        // If we don't want to create a file when opening, the lookup must succeed.
        // If we want to create a temporary unnamed file, `path` is the directory of the owning FS.
        // In that case the path also has to exist.
        if !flags.contains(OpenFlags::Create) || flags.contains(OpenFlags::Temporary) {
            lookup_flags |= LookupFlags::MustExist;
        }
        if flags.contains(OpenFlags::Exclusive) {
            lookup_flags |= LookupFlags::MustNotExist;
        }
        if !flags.intersects(OpenFlags::Exclusive | OpenFlags::NoFollow) {
            lookup_flags |= LookupFlags::FollowSymlinks;
        }

        let file_path = PathNode::flookup(proc, at, path, identity, lookup_flags)?;
        match file_path.entry.get_inode() {
            Some(x) => Self::do_open_inode(file_path, &x, flags, identity),
            None => {
                // If the lookup was successful, we expect that the entry is positive.
                if !flags.contains(OpenFlags::Create) {
                    warn!("Tried opening a file without O_CREAT and backing inode, this is a bug!");
                    return Err(Errno::ENOENT);
                }

                let parent = file_path
                    .lookup_parent()
                    .and_then(|p| p.entry.get_inode().ok_or(Errno::ENOENT))
                    .expect("Entry should always have a parent");

                match &parent.node_ops {
                    NodeOps::Directory(_) => (),
                    _ => return Err(Errno::ENOTDIR),
                }

                parent.try_access(identity, flags, false)?;

                let file_node = parent.sb.clone().create_inode(NodeType::Regular, mode)?;
                file_path.entry.as_ref().set_inode(file_node.clone());
                let result = File {
                    path: Some(file_path),
                    ops: file_node.file_ops.clone(),
                    inode: Some(file_node),
                    flags,
                    position: AtomicUsize::new(0),
                };

                Ok(Arc::try_new(result)?)
            }
        }
    }

    pub fn open_inode(
        path: PathNode,
        inode: &Arc<INode>,
        flags: OpenFlags,
        identity: &Identity,
    ) -> EResult<Arc<Self>> {
        Self::do_open_inode(path, inode, flags, identity)
    }

    fn do_open_inode(
        file_path: PathNode,
        inode: &Arc<INode>,
        flags: OpenFlags,
        identity: &Identity,
    ) -> EResult<Arc<Self>> {
        // If we want to open as a directory, make sure this is actually a directory.
        if flags.contains(OpenFlags::Directory) {
            match &inode.node_ops {
                NodeOps::Directory(_) => {}
                _ => return Err(Errno::ENOTDIR),
            }
        }

        match &inode.node_ops {
            NodeOps::Regular(_) => {
                inode.try_access(identity, flags, false)?;

                let result = File {
                    path: Some(file_path),
                    ops: inode.file_ops.clone(),
                    inode: Some(inode.clone()),
                    flags,
                    position: AtomicUsize::new(0),
                };

                Ok(Arc::try_new(result)?)
            }
            NodeOps::Directory(dir) => dir.open(inode, file_path, flags, identity),
            NodeOps::BlockDevice(blk) => blk.open(),
            NodeOps::CharacterDevice(chr) => chr.open(),
            NodeOps::FIFO => todo!(),
            NodeOps::SymbolicLink(_) => return Err(Errno::ELOOP),
            // Doesn't make sense to call open() on anything else.
            _ => return Err(Errno::ENOTSUP),
        }
    }

    /// Reads directory entries into a buffer.
    /// Returns actual bytes read.
    pub fn read_dir(&self, buf: &mut [u8]) -> EResult<isize> {
        if buf.is_empty() {
            return Ok(0);
        }
        self.ops.read(self, buf, 0)
    }

    /// Reads into a buffer from a file.
    /// Returns actual bytes read.
    pub fn read(&self, buf: &mut [u8]) -> EResult<isize> {
        if buf.is_empty() {
            return Ok(0);
        }
        self.ops
            .read(self, buf, self.position.load(Ordering::Acquire) as _)
    }

    /// Reads into a buffer from a file at a specified offset.
    /// Returns actual bytes read.
    pub fn pread(&self, buf: &mut [u8], offset: u64) -> EResult<isize> {
        if buf.is_empty() {
            return Ok(0);
        }
        self.ops.read(self, buf, offset as _)
    }

    /// Writes a buffer to a file.
    /// Returns actual bytes written.
    pub fn write(&self, buf: &[u8]) -> EResult<isize> {
        if buf.is_empty() {
            return Ok(0);
        }
        self.ops
            .write(self, buf, self.position.load(Ordering::Acquire) as _)
    }

    /// Writes a buffer to a file at a specified offset.
    /// Returns actual bytes written.
    pub fn pwrite(&self, buf: &[u8], offset: u64) -> EResult<isize> {
        if buf.is_empty() {
            return Ok(0);
        }
        self.ops.write(self, buf, offset as _)
    }

    pub fn poll(&self, mask: u16) -> EResult<u16> {
        self.ops.poll(self, mask)
    }

    pub fn seek(&self, offset: SeekAnchor) -> EResult<uapi::off_t> {
        self.ops.seek(self, offset)
    }

    pub fn ioctl(&self, request: usize, arg: usize) -> EResult<usize> {
        self.ops.ioctl(self, request, arg)
    }

    /// If a private mapping is requested, creates a new memory object and copies the data over.
    pub fn get_memory_object(
        &self,
        length: NonZeroUsize,
        offset: uapi::off_t,
        private: bool,
    ) -> EResult<Arc<MemoryObject>> {
        let cache = self
            .inode
            .as_ref()
            .ok_or(Errno::ENOENT)
            .and_then(|x| Ok(x.cache.clone()))?;

        if private {
            // Private mapping means we need to do a unique allocation.
            let phys = MemoryObject::new_phys();
            let mut buf = vec![0u8; length.into()];
            cache.read(&mut buf, offset as _);
            phys.write(&buf, offset as _);
            Arc::try_new(phys).map_err(|_| Errno::ENOMEM)
        } else {
            Ok(cache)
        }
    }
}
