use super::inode::INode;
use crate::generic::{
    memory::{VirtAddr, virt::AddressSpace},
    posix::errno::EResult,
    util::mutex::Mutex,
    vfs::path::PathBuf,
};
use alloc::sync::Arc;
use core::{fmt::Debug, sync::atomic::AtomicUsize};

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
    }
}

pub enum SeekAnchor {
    /// Seek relative to the start of the file.
    Start,
    /// Seek relative to the current cursor position.
    Current,
    /// Seek relative to the end of the file.
    End,
}

/// The kernel representation of an open file.
pub struct File {
    /// The underlying inode that this file is pointing to.
    pub inode: Arc<INode>,
    /// The current position of the cursor in this file.
    pub position: AtomicUsize,
    /// File open flags.
    pub flags: Mutex<OpenFlags>,
}

/// Operations that can be performed on a file.
pub trait FileOps: Debug {
    /// Reads from the file into a buffer.
    /// Returns actual bytes read and the new offset.
    fn read(&self, file: &File, buffer: &mut [u8]) -> EResult<usize>;

    /// Writes a buffer to the file.
    /// Returns actual bytes written.
    fn write(&self, file: &File, buffer: &[u8]) -> EResult<usize>;

    /// Seeks inside the file.
    /// Returns the new absolute offset.
    fn seek(&self, file: &File, offset: i64, whence: SeekAnchor) -> EResult<usize>;

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
    /// Opens a file identified by a path.
    pub fn open(
        relative_to: &Self,
        path: PathBuf, // TODO: This doesn't have to be an owned value.
        flags: OpenFlags,
        mode: uapi::mode_t,
    ) -> EResult<Arc<Self>> {
        todo!()
    }

    /// Reads directory entries into a buffer. Returns actual bytes read.
    pub fn read_dir(&self, buf: &mut [u8]) -> EResult<u64> {
        todo!()
    }

    /// Reads into a buffer from a file. Returns actual bytes read.
    pub fn read(&self, buf: &mut [u8]) -> EResult<u64> {
        todo!()
    }

    /// Reads into a buffer from a file at a specified offset. Returns actual bytes read.
    pub fn pread(&self, buf: &mut [u8], offset: u64) -> EResult<u64> {
        todo!()
    }

    /// Writes a buffer to a file. Returns actual bytes written.
    pub fn write(&self, buf: &[u8]) -> EResult<u64> {
        todo!()
    }

    /// Writes a buffer to a file at a specified offset. Returns actual bytes written.
    pub fn pwrite(&self, buf: &[u8], offset: u64) -> EResult<u64> {
        todo!()
    }

    pub fn seek(&self, offset: i64, whence: SeekAnchor) -> EResult<u64> {
        todo!()
    }
}
