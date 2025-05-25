pub(super) mod initrd;
pub(super) mod tmpfs;

use super::inode::INode;
use crate::generic::{posix::errno::EResult, util::mutex::Mutex};
use alloc::{collections::btree_map::BTreeMap, sync::Arc};
use core::fmt::Debug;

#[derive(Debug)]
pub struct FileSystem {
    name: &'static str,
    ops: &'static dyn FileSystemOps,
}

impl FileSystem {
    pub fn new(name: &'static str, ops: &'static dyn FileSystemOps) -> Self {
        Self { name, ops }
    }
}

pub trait FileSystemOps: Debug {
    /// Gets the status of the file system.
    fn statvfs(&self) -> EResult<uapi::statvfs>;

    /// Synchronizes the entire file system.
    fn sync(&self) -> EResult<()>;
}

#[derive(Debug)]
pub struct SuperBlock {}

pub trait SuperBlockOps {
    fn create_inode(&self, sb: SuperBlock) -> Arc<INode>;
    fn destroy_inode(&self, inode: Arc<INode>);
    fn dirty_inode(&self, inode: Arc<INode>);
}

/// A map of all known and registered file systems.
static FS_TABLE: Mutex<BTreeMap<&'static str, FileSystem>> = Mutex::new(BTreeMap::new());
