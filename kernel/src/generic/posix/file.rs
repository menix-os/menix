use super::errno::{EResult, Errno};
use crate::generic::memory::virt::ForeignPtr;

pub struct File {
    ops: FileOps,
}

/// Operations that can be performed on a file.
#[derive(Default, Debug)]
pub struct FileOps {
    /// Reads from the file into a buffer.
    /// Returns actual bytes read.
    read: Option<fn(file: &mut File, buffer: &mut [u8]) -> EResult<usize>>,

    /// Writes a buffer to the file.
    /// Returns actual bytes written.
    write: Option<fn(file: &mut File, buffer: &[u8]) -> EResult<usize>>,

    /// Reads from the file, starting at `offset`, into a buffer.
    /// Returns actual bytes read.
    pread: Option<fn(file: &mut File, buffer: &mut [u8], offset: isize) -> EResult<usize>>,

    /// Writes a buffer to the file, starting at `offset`.
    /// Returns actual bytes written.
    pwrite: Option<fn(file: &mut File, buffer: &[u8], offset: isize) -> EResult<usize>>,

    /// Seeks inside the file, relative to the start of the file.
    /// Returns the new absolute offset.
    seek_set: Option<fn(file: &mut File, offset: isize) -> EResult<usize>>,

    /// Seeks inside the file, relative to the current position.
    /// Returns the new absolute offset.
    seek_cur: Option<fn(file: &mut File, offset: isize) -> EResult<usize>>,

    /// Seeks inside the file, relative to the end of the file.
    /// Returns the new absolute offset.
    seek_end: Option<fn(file: &mut File, offset: isize) -> EResult<usize>>,

    /// Performs a generic ioctl operation on the file.
    /// Returns a status code.
    ioctl: Option<fn(file: &mut File, request: usize, arg: ForeignPtr<u8>) -> EResult<usize>>,
    // TODO: Add the rest of the operations.
}
