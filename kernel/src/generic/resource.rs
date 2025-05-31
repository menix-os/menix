use super::posix::errno::{EResult, Errno};

pub trait Resource: Sync + Send {
    /// Reads up to `buffer.len()` bytes from the file into `buffer`, starting at `offset`.
    /// Returns actual amount of bytes read.
    fn read(&self, offset: usize, buffer: &mut [u8]) -> EResult<usize> {
        _ = buffer;
        return Err(Errno::ENOSYS);
    }

    /// Writes a buffer to the resource starting at `offset`.
    /// Returns actual amount of bytes written.
    fn write(&self, offset: usize, buffer: &[u8]) -> EResult<usize> {
        _ = buffer;
        return Err(Errno::ENOSYS);
    }

    /// Maps the resource in virtual memory.
    fn mmap(&self /* TODO */) -> EResult<()> {
        return Err(Errno::ENOSYS);
    }

    /// Performs a generic ioctl operation on the file.
    /// Returns the result of the ioctl request.
    fn ioctl(&self, request: usize, arg: usize) -> EResult<usize> {
        _ = (request, arg);
        return Err(Errno::ENOSYS);
    }
}
