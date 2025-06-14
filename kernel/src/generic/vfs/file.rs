use super::inode::INode;
use crate::generic::{
    memory::{VirtAddr, virt::AddressSpace},
    posix::errno::{EResult, Errno},
    process::Identity,
    vfs::{
        entry::Entry,
        inode::{Mode, NodeOps},
        path::PathBuf,
    },
};
use alloc::sync::Arc;
use core::fmt::Debug;

bitflags::bitflags! {
    pub struct OpenFlags: u32 {
        /// Create the file if it's missing.
        const Create = uapi::O_CREAT as u32;
        /// Exclusive use.
        const Exclusive = uapi::O_EXCL as u32;
        /// Do not assign a controlling terminal.
        const NoCtrlTerminal = uapi::O_NOCTTY as u32;
        const Truncate = uapi::O_TRUNC as u32;
        const Append = uapi::O_APPEND as u32;
        const NonBlocking = uapi::O_NONBLOCK as u32;
        const SyncData = uapi::O_DSYNC as u32;
        const Directory = uapi::O_DIRECTORY as u32;
        /// Don't follow symbolic links.
        const NoFollow = uapi::O_NOFOLLOW as u32;
        const CloseOnExec = uapi::O_CLOEXEC as u32;
        const Sync = uapi::O_SYNC as u32;
        const SyncRead = uapi::O_RSYNC as u32;
        const LargeFile = uapi::O_LARGEFILE as u32;
        /// Don't update the access time.
        const NoAccessTime = uapi::O_NOATIME as u32;
        const Temporary = uapi::O_TMPFILE as u32;
        const ReadOnly = uapi::O_RDONLY as u32;
        const WriteOnly = uapi::O_WRONLY as u32;
        const ReadWrite = uapi::O_RDWR as u32;
        const Executeable = uapi::O_EXEC as u32;
    }
}

pub enum SeekAnchor {
    /// Seek relative to the start of the file.
    Start(i64),
    /// Seek relative to the current cursor position.
    Current(i64),
    /// Seek relative to the end of the file.
    End(i64),
}

/// The kernel representation of an open file.
pub struct File {
    /// The underlying inode that this file is pointing to.
    inode: Arc<INode>,
    /// The cached entry for this node.
    entry: Arc<Entry>,
    /// Operations that can be performed on this file.
    ops: Arc<dyn FileOps>,
    /// File open flags.
    flags: OpenFlags,
}

/// Operations that can be performed on a file.
pub trait FileOps: Debug {
    /// Reads directory entries into a buffer.
    /// Returns actual bytes read.
    fn read_dir(&self, file: &File, buffer: &mut [u8]) -> EResult<u64>;

    /// Reads from the file into a buffer.
    /// Returns actual bytes read and the new offset.
    fn read(&self, file: &File, buffer: &mut [u8], offset: SeekAnchor) -> EResult<u64>;

    /// Writes a buffer to the file.
    /// Returns actual bytes written.
    fn write(&self, file: &File, buffer: &[u8], offset: SeekAnchor) -> EResult<u64>;

    /// Seeks inside the file.
    /// Returns the new absolute offset.
    fn seek(&self, file: &File, offset: SeekAnchor) -> EResult<u64>;

    /// Performs a generic ioctl operation on the file.
    /// Returns a status code.
    fn ioctl(&self, file: &File, request: usize, arg: usize) -> EResult<usize>;

    /// Maps a file from an `offset` into the given address space.
    fn mmap(
        &self,
        file: &File,
        space: &AddressSpace,
        offset: u64,
        hint: VirtAddr,
        size: usize,
    ) -> EResult<VirtAddr>;
}

impl File {
    /// Opens a file referenced by a path for a given `identity`.
    pub fn open(
        path: &PathBuf,
        at: Option<&Self>,
        flags: OpenFlags,
        identity: &Identity,
    ) -> EResult<Arc<Self>> {
        todo!()
    }

    /// Reads directory entries into a buffer.
    /// Returns actual bytes read.
    pub fn read_dir(&self, buf: &mut [u8]) -> EResult<u64> {
        self.ops.read_dir(self, buf)
    }

    /// Reads into a buffer from a file.
    /// Returns actual bytes read.
    pub fn read(&self, buf: &mut [u8]) -> EResult<u64> {
        self.ops.read(self, buf, SeekAnchor::Current(0))
    }

    /// Reads into a buffer from a file at a specified offset.
    /// Returns actual bytes read.
    pub fn pread(&self, buf: &mut [u8], offset: i64) -> EResult<u64> {
        self.ops.read(self, buf, SeekAnchor::Start(offset))
    }

    /// Writes a buffer to a file.
    /// Returns actual bytes written.
    pub fn write(&self, buf: &[u8]) -> EResult<u64> {
        self.ops.write(self, buf, SeekAnchor::Current(0))
    }

    /// Writes a buffer to a file at a specified offset.
    /// Returns actual bytes written.
    pub fn pwrite(&self, buf: &[u8], offset: i64) -> EResult<u64> {
        self.ops.write(self, buf, SeekAnchor::Start(offset))
    }

    pub fn seek(&self, offset: SeekAnchor) -> EResult<u64> {
        self.ops.seek(self, offset)
    }

    pub fn ioctl(&self, request: usize, arg: usize) -> EResult<usize> {
        self.ops.ioctl(self, request, arg)
    }

    pub fn mmap(
        &self,
        space: &AddressSpace,
        offset: u64,
        hint: VirtAddr,
        size: usize,
    ) -> EResult<VirtAddr> {
        self.ops.mmap(self, space, offset, hint, size)
    }

    fn open_entry(
        &self,
        entry: &Entry,
        inode: Arc<INode>,
        flags: OpenFlags,
        identity: &Identity,
    ) -> EResult<Arc<Self>> {
        if flags.contains(OpenFlags::Directory) {
            if let NodeOps::Directory(x) = &inode.node_ops {
                return Err(Errno::ENOTDIR);
            }
        }

        // Check if we are allowed to access this node.
        inode.try_access(identity, flags)?;

        match &inode.node_ops {
            NodeOps::Regular(reg) => {
                todo!();
            }
            NodeOps::Directory(dir) => {
                todo!();
            }
            // Symbolic links can't be opened.
            NodeOps::SymbolicLink(_) => return Err(Errno::ELOOP),
            NodeOps::FIFO => todo!(),
            NodeOps::BlockDevice => todo!(),
            NodeOps::CharacterDevice => todo!(),
            NodeOps::Socket => todo!(),
        }

        todo!()
    }
}
