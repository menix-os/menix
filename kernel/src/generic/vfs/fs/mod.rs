mod initrd;
mod tmpfs;

use super::inode::INode;
use crate::generic::{
    posix::errno::{EResult, Errno},
    util::mutex::Mutex,
    vfs::{
        entry::{Entry, Mount, MountFlags},
        inode::Mode,
    },
};
use alloc::{collections::btree_map::BTreeMap, string::String, sync::Arc};
use core::fmt::Debug;

pub trait FileSystem: Debug {
    /// Returns an identifier which can be used to determine this file system.
    fn get_name(&self) -> &'static [u8];

    /// Mounts an instance of this file system from a `source`.
    /// Returns a reference to the mount point with an instance of this file system.
    /// Some file systems don't require a `source` and may ignore the argument.
    fn mount(&self, source: Option<Arc<Entry>>, flags: MountFlags) -> EResult<Arc<Mount>>;
}

/// A super block is the control structure of a file system instance.
/// It manages inodes.
pub trait SuperBlock: Debug {
    /// Synchronizes the entire file system.
    fn sync(self: Arc<Self>) -> EResult<()>;

    /// Gets the status of the file system.
    fn statvfs(self: Arc<Self>) -> EResult<uapi::statvfs>;

    /// Allocates a new inode on this super block.
    fn create_inode(self: Arc<Self>, mode: Mode) -> EResult<Arc<INode>>;

    /// Deletes the inode.
    fn destroy_inode(self: Arc<Self>, inode: INode) -> EResult<()>;
}

/// A map of all known and registered file systems.
static FS_TABLE: Mutex<BTreeMap<&'static [u8], &'static dyn FileSystem>> =
    Mutex::new(BTreeMap::new());

/// Registers a new file system.
pub fn register_fs(fs: &'static dyn FileSystem) {
    let name = fs.get_name();
    FS_TABLE.lock().insert(name, fs);
    log!(
        "Registered new file system \"{}\"",
        String::from_utf8_lossy(name)
    );
}

/// Mounts a file system at path `source` on `target`.
pub fn mount(source: Option<Arc<Entry>>, fs_name: &[u8], flags: MountFlags) -> EResult<Arc<Mount>> {
    let table = FS_TABLE.lock();
    let fs = table.get(fs_name).ok_or(Errno::ENODEV)?;
    fs.mount(source, flags)
}
