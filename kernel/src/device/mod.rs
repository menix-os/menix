use crate::{
    memory::VirtAddr,
    posix::errno::EResult,
    vfs::{File, file::FileOps},
};

mod console;
pub mod input;
mod memfiles;

/// Represents a generic character device.
pub trait CharDevice: FileOps {
    fn name(&self) -> &str;
}

pub trait BlockDevice {
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

    fn handle_ioctl(&self, file: &File, request: usize, arg: VirtAddr) -> EResult<usize>;
}

impl FileOps for dyn BlockDevice {
    fn read(&self, file: &File, buffer: &mut [u8], offset: u64) -> EResult<isize> {
        let _ = (offset, buffer, file);
        todo!()
    }

    fn write(&self, file: &File, buffer: &[u8], offset: u64) -> EResult<isize> {
        let _ = (offset, buffer, file);
        todo!()
    }

    fn ioctl(&self, file: &File, request: usize, arg: VirtAddr) -> EResult<usize> {
        self.handle_ioctl(file, request, arg)
    }

    fn poll(&self, file: &File, mask: u16) -> EResult<u16> {
        _ = (file, mask);
        Ok(mask)
    }
}
