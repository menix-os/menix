use crate::{
    device::Device,
    posix::errno::{EResult, Errno},
    process::{Identity, PROCESS_STAGE, Process},
    vfs::{
        self, File, VFS_DEV_MOUNT_STAGE,
        file::FileOps,
        inode::{Mode, NodeType},
    },
};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct NullFile;

impl FileOps for NullFile {
    fn read(&self, _: &File, _: &mut [u8], _: u64) -> EResult<isize> {
        Ok(0)
    }

    fn write(&self, _: &File, buffer: &[u8], _: u64) -> EResult<isize> {
        Ok(buffer.len() as _)
    }
}

impl Device for NullFile {
    fn open(&self) -> EResult<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        "null"
    }
}

#[derive(Debug)]
pub struct ZeroFile;

impl FileOps for ZeroFile {
    fn read(&self, _: &File, buffer: &mut [u8], _: u64) -> EResult<isize> {
        buffer.fill(0);
        Ok(buffer.len() as _)
    }

    fn write(&self, _: &File, buffer: &[u8], _: u64) -> EResult<isize> {
        Ok(buffer.len() as _)
    }
}

impl Device for ZeroFile {
    fn open(&self) -> EResult<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        "zero"
    }
}

#[derive(Debug)]
pub struct FullFile;

impl FileOps for FullFile {
    fn read(&self, _: &File, buffer: &mut [u8], _: u64) -> EResult<isize> {
        buffer.fill(0);
        Ok(buffer.len() as _)
    }

    fn write(&self, _: &File, _: &[u8], _: u64) -> EResult<isize> {
        Err(Errno::ENOSPC)
    }
}

impl Device for FullFile {
    fn open(&self) -> EResult<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        "full"
    }
}

#[initgraph::task(
    name = "generic.device.memfiles",
    depends = [PROCESS_STAGE, VFS_DEV_MOUNT_STAGE]
)]
fn MEMFILES_STAGE() {
    let inner = Process::get_kernel().inner.lock();

    vfs::mknod(
        &inner,
        None,
        b"/dev/null",
        NodeType::CharacterDevice,
        Mode::from_bits_truncate(0o666),
        Some(Arc::new(NullFile)),
        Identity::get_kernel(),
    )
    .expect("Unable to create /dev/null");

    vfs::mknod(
        &inner,
        None,
        b"/dev/full",
        NodeType::CharacterDevice,
        Mode::from_bits_truncate(0o666),
        Some(Arc::new(FullFile)),
        Identity::get_kernel(),
    )
    .expect("Unable to create /dev/full");

    vfs::mknod(
        &inner,
        None,
        b"/dev/zero",
        NodeType::CharacterDevice,
        Mode::from_bits_truncate(0o666),
        Some(Arc::new(ZeroFile)),
        Identity::get_kernel(),
    )
    .expect("Unable to create /dev/zero");
}
