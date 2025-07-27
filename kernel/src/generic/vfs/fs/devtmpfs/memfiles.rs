use crate::generic::{
    posix::errno::{EResult, Errno},
    vfs::{
        File,
        file::{FileOps, SeekAnchor},
    },
};

/// Represents `/dev/null`. Consumes all input, and returns 0 bytes.
#[derive(Debug)]
pub struct NullFile;

impl FileOps for NullFile {
    fn seek(&self, _: &File, _: SeekAnchor) -> EResult<uapi::off_t> {
        Ok(0)
    }

    fn read(&self, _: &File, buffer: &mut [u8], _: uapi::off_t) -> EResult<isize> {
        buffer.fill(0);
        Ok(buffer.len() as _)
    }

    fn write(&self, _: &File, buffer: &[u8], _: uapi::off_t) -> EResult<isize> {
        Ok(buffer.len() as _)
    }

    fn ioctl(&self, file: &File, request: usize, arg: usize) -> EResult<usize> {
        _ = (arg, request, file);
        Err(Errno::ENOTTY)
    }

    fn poll(&self, file: &File, mask: u16) -> EResult<u16> {
        _ = (file, mask);
        Ok(mask)
    }
}
