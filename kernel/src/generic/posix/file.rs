use super::errno::{EResult, Errno};
use crate::generic::memory::virt::ForeignPtr;

/// Operations that can be performed on a file.
pub trait FileOps {
    /// Reads from the file into a buffer.
    /// Returns actual bytes read.
    fn read(&mut self, buffer: &mut [u8]) -> EResult<usize> {
        Err(Errno::EINVAL)
    }

    /// Writes a buffer to the file.
    /// Returns actual bytes written.
    fn write(&mut self, buffer: &[u8]) -> EResult<usize> {
        Err(Errno::EINVAL)
    }

    /// Reads from the file, starting at `offset`, into a buffer.
    /// Returns actual bytes read.
    fn pread(&mut self, buffer: &mut [u8], offset: isize) -> EResult<usize> {
        Err(Errno::EINVAL)
    }

    /// Writes a buffer to the file, starting at `offset`.
    /// Returns actual bytes written.
    fn pwrite(&mut self, buffer: &[u8], offset: isize) -> EResult<usize> {
        Err(Errno::EINVAL)
    }

    /// Seeks inside the file, relative to the start of the file.
    /// Returns the new absolute offset.
    fn seek_set(&mut self, offset: isize) -> EResult<usize> {
        Err(Errno::EINVAL)
    }

    /// Seeks inside the file, relative to the current position.
    /// Returns the new absolute offset.
    fn seek_cur(&mut self, offset: isize) -> EResult<usize> {
        Err(Errno::EINVAL)
    }

    /// Seeks inside the file, relative to the end of the file.
    /// Returns the new absolute offset.
    fn seek_end(&mut self, offset: isize) -> EResult<usize> {
        Err(Errno::EINVAL)
    }

    /// Performs a generic ioctl operation on the file.
    /// Returns a status code.
    fn ioctl(&mut self, request: usize, arg: ForeignPtr<u8>) -> EResult<usize> {
        Err(Errno::EINVAL)
    }

    // TODO: Add the rest of the operations.
}
