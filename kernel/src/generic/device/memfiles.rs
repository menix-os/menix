use crate::generic::{
    posix::errno::{EResult, Errno},
    vfs::{
        File,
        file::{FileOps, SeekAnchor},
    },
};

#[derive(Debug)]
pub struct NullFile;

impl FileOps for NullFile {
    fn seek(&self, _: &File, _: SeekAnchor) -> EResult<uapi::off_t> {
        Ok(0)
    }

    fn read(&self, _: &File, _: &mut [u8], _: uapi::off_t) -> EResult<isize> {
        Ok(0)
    }

    fn write(&self, _: &File, buffer: &[u8], _: uapi::off_t) -> EResult<isize> {
        Ok(buffer.len() as _)
    }
}

#[derive(Debug)]
pub struct ZeroFile;

impl FileOps for ZeroFile {
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
}

#[derive(Debug)]
pub struct FullFile;

impl FileOps for FullFile {
    fn seek(&self, _: &File, _: SeekAnchor) -> EResult<uapi::off_t> {
        Ok(0)
    }

    fn read(&self, _: &File, buffer: &mut [u8], _: uapi::off_t) -> EResult<isize> {
        buffer.fill(0);
        Ok(buffer.len() as _)
    }

    fn write(&self, _: &File, _: &[u8], _: uapi::off_t) -> EResult<isize> {
        Err(Errno::ENOSPC)
    }
}
