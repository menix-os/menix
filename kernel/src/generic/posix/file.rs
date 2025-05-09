use super::errno::{EResult, Errno};
use crate::generic::memory::user::UserBuffer;

/// Operations that can be performed on a file.
pub trait FileOps {
    /// Reads from the file into a buffer.
    /// Returns actual bytes read.
    fn read(&mut self, buffer: &mut [u8]) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    /// Writes a buffer to the file.
    /// Returns actual bytes written.
    fn write(&mut self, buffer: &[u8]) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    /// Reads from the file, starting at `offset`, into a buffer.
    /// Returns actual bytes read.
    fn pread(&mut self, buffer: &mut [u8], offset: isize) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    /// Writes a buffer to the file, starting at `offset`.
    /// Returns actual bytes written.
    fn pwrite(&mut self, buffer: &[u8], offset: isize) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    /// Seeks inside the file, relative to the start of the file.
    /// Returns the new absolute offset.
    fn seek_set(&mut self, offset: isize) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    /// Seeks inside the file, relative to the current position.
    /// Returns the new absolute offset.
    fn seek_cur(&mut self, offset: isize) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    /// Seeks inside the file, relative to the end of the file.
    /// Returns the new absolute offset.
    fn seek_end(&mut self, offset: isize) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    /// Performs a generic ioctl operation on the file.
    /// Returns a status code.
    fn ioctl(&mut self, request: usize, arg: UserBuffer<u8>) -> EResult<usize> {
        return Err(Errno::ENOSYS);
    }

    // TODO: Add the rest of the operations.
}
