use super::inode::INode;
use crate::generic::posix::errno::{EResult, Errno};
use alloc::{boxed::Box, sync::Arc};
use core::sync::atomic::AtomicUsize;

/// The kernel representation of an open file.
pub struct File {
    /// File system operations to call on this file.
    ops: Box<dyn FileOps>,
    /// The underlying inode that this file is pointing to.
    inode: Arc<INode>,
    /// The current position of the cursor in this file.
    position: AtomicUsize,
}

impl File {
    pub fn read(&self, buffer: &mut [u8]) -> EResult<(usize, isize)> {
        self.ops.read(self, buffer)
    }
}

/// Operations that can be performed on a file.
pub trait FileOps {
    /// Reads from the file into a buffer.
    /// Returns actual bytes read and the new offset.
    fn read(&self, file: &File, buffer: &mut [u8]) -> EResult<(usize, isize)> {
        return Err(Errno::ENOSYS);
    }

    /// Writes a buffer to the file.
    /// Returns actual bytes written.
    fn write(&self, file: &File, buffer: &[u8]) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    /// Seeks inside the file.
    /// Returns the new absolute offset.
    fn seek(&self, file: &File, offset: isize, whence: isize) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    /// Performs a generic ioctl operation on the file.
    /// Returns a status code.
    fn ioctl(&self, file: &File, request: usize, arg: usize) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    // TODO: Add the rest of the operations.
}
