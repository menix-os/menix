use super::node::Node;
use crate::generic::posix::errno::{EResult, Errno};
use alloc::sync::Arc;

/// The kernel representation of an open file descriptor.
pub struct File {
    ops: &'static dyn FileOps,
    node: Arc<Node>,
}

impl File {
    pub fn read(&mut self, buffer: &mut [u8]) -> EResult<(usize, isize)> {
        self.ops.read(self, buffer)
    }
}

/// Operations that can be performed on a file.
pub trait FileOps {
    /// Reads from the file into a buffer.
    /// Returns actual bytes read and the new offset.
    fn read(&self, file: &mut File, buffer: &mut [u8]) -> EResult<(usize, isize)> {
        return Err(Errno::ENOSYS);
    }

    /// Writes a buffer to the file.
    /// Returns actual bytes written.
    fn write(&self, file: &mut File, buffer: &[u8]) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    /// Seeks inside the file.
    /// Returns the new absolute offset.
    fn seek(&self, file: &mut File, offset: isize, whence: isize) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    /// Performs a generic ioctl operation on the file.
    /// Returns a status code.
    fn ioctl(&self, file: &mut File, request: usize, arg: usize) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    // TODO: Add the rest of the operations.
}
