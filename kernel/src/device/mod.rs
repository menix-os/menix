use crate::{posix::errno::EResult, vfs::file::FileOps};

mod console;
pub mod input;
mod memfiles;

/// Represents a generic character device.
pub trait CharDevice: FileOps {
    fn name(&self) -> &str;
}

pub trait BlockDevice: FileOps {
    /// Gets the size of a sector in bytes.
    fn get_sector_size(&self) -> usize;

    /// Reads enough sectors to fill the buffer from `sector_start`.
    /// # Safety
    /// The implementation must allow `buffer` to have any position and size.
    fn read_sectors(&self, buffer: &mut [u8], sector_start: usize) -> EResult<usize>;

    /// Writes the buffer to the device starting at `sector_start`.
    /// # Safety
    /// The implementation must allow `buffer` to have any position and size.
    fn write_sectors(&self, buffer: &[u8], sector_start: usize) -> EResult<usize>;
}
