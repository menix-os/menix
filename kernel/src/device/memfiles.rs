use crate::{
    device::CharDevice,
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

impl CharDevice for NullFile {
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

impl CharDevice for ZeroFile {
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

impl CharDevice for FullFile {
    fn name(&self) -> &str {
        "full"
    }
}

#[initgraph::task(
    name = "generic.device.memfiles",
    depends = [PROCESS_STAGE, VFS_DEV_MOUNT_STAGE]
)]
fn MEMFILES_STAGE() {
    let proc = Process::get_kernel();
    let root = proc.root_dir.lock();
    let cwd = proc.working_dir.lock();

    vfs::mknod(
        root.clone(),
        cwd.clone(),
        b"/dev/null",
        NodeType::CharacterDevice,
        Mode::from_bits_truncate(0o666),
        Some(Arc::new(NullFile)),
        Identity::get_kernel(),
    )
    .expect("Unable to create /dev/null");

    vfs::mknod(
        root.clone(),
        cwd.clone(),
        b"/dev/full",
        NodeType::CharacterDevice,
        Mode::from_bits_truncate(0o666),
        Some(Arc::new(FullFile)),
        Identity::get_kernel(),
    )
    .expect("Unable to create /dev/full");

    vfs::mknod(
        root.clone(),
        cwd.clone(),
        b"/dev/zero",
        NodeType::CharacterDevice,
        Mode::from_bits_truncate(0o666),
        Some(Arc::new(ZeroFile)),
        Identity::get_kernel(),
    )
    .expect("Unable to create /dev/zero");
}
