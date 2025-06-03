pub(super) mod initrd;
pub(super) mod tmpfs;

use super::{entry::Entry, inode::INode};
use crate::generic::{posix::errno::EResult, util::mutex::Mutex};
use alloc::{boxed::Box, collections::btree_map::BTreeMap, sync::Arc};
use core::fmt::Debug;

pub trait FileSystem: Debug {
    /// Returns an identifier which can be used to determine this file system.
    fn get_name(&self) -> &'static str;

    /// Mounts an instance of this file system on a `mount_point`.
    /// Returns a reference to the super block and the root of the file system.
    fn mount(&self, mount_point: Arc<Entry>) -> EResult<(Arc<dyn SuperBlock>, Arc<INode>)>;
}

/// A super block is the control structure of a file system instance.
/// It provides operations to create, modify and delete inodes.
pub trait SuperBlock: Debug {
    /// Unmounts this super block.
    fn unmount(&self) -> EResult<()>;
    /// Gets the status of the file system.
    fn statvfs(&self) -> EResult<uapi::statvfs>;
    /// Synchronizes the entire file system.
    fn sync(&self) -> EResult<()>;
    /// Allocates a new inode on this super block.
    // TODO: Split into NodeType and Mode
    fn create_inode(&self, mode: uapi::mode_t) -> EResult<Arc<INode>>;
    /// Deletes the inode.
    fn destroy_inode(&self, inode: INode) -> EResult<()>;
}

/// Registers a new file system.
pub fn register_fs(fs: Box<dyn FileSystem>) {
    let name = fs.get_name();
    FS_TABLE.lock().insert(name, fs);
    log!("Registered new file system \"{}\"", name);
}

/// A map of all known and registered file systems.
static FS_TABLE: Mutex<BTreeMap<&'static str, Box<dyn FileSystem>>> = Mutex::new(BTreeMap::new());
