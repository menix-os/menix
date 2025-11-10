use crate::{
    posix::errno::EResult,
    util::once::Once,
    vfs::{Entry, Mount, MountFlags, fs::FileSystem},
};
use alloc::sync::Arc;

static DEV_MOUNT: Once<Arc<Mount>> = Once::new();

#[derive(Debug)]
struct DevTmpFs;

impl FileSystem for DevTmpFs {
    fn get_name(&self) -> &'static [u8] {
        b"devtmpfs"
    }

    fn mount(&self, _: Option<Arc<Entry>>, _: MountFlags) -> EResult<Arc<super::Mount>> {
        Ok(DEV_MOUNT.get().clone())
    }
}

#[initgraph::task(
    name = "generic.vfs.devtmpfs",
    depends = [super::tmpfs::TMPFS_INIT_STAGE],
    entails = [crate::vfs::VFS_STAGE],
)]
pub fn DEVTMPFS_STAGE() {
    super::register_fs(&DevTmpFs);

    // Ask for a singleton-like tmpfs.
    let tmpfs = super::mount(None, b"tmpfs", MountFlags::empty())
        .expect("Unable to create devtmpfs from tmpfs");

    unsafe { DEV_MOUNT.init(tmpfs) };
}
