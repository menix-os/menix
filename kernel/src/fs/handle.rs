use crate::system::error::Errno;

// Handle for managing input/output streams.
use super::fd::FileDescriptor;

pub trait Handle: Send {
    /// Tries to read `output.len()` bytes at `offset` into `output`. Returns actual bytes read.
    fn read(
        &mut self,
        fd: Option<&FileDescriptor>,
        output: &mut [u8],
        offset: usize,
    ) -> Result<usize, Errno>;

    /// Tries to write `output.len()` bytes at `offset` from `input`. Returns actual bytes read.
    fn write(
        &mut self,
        fd: Option<&FileDescriptor>,
        input: &[u8],
        offset: usize,
    ) -> Result<usize, Errno>;

    /// Executes an IO control `request` with an `argument` with an implementation defined function on `fd`.
    fn ioctl(
        &mut self,
        fd: Option<&FileDescriptor>,
        request: u32,
        argument: usize,
    ) -> Result<usize, Errno>;
}
