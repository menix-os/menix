pub(super) mod initrd;
pub(super) mod tmpfs;

use super::{entry::Entry, inode::INode};
use crate::generic::{posix::errno::EResult, util::mutex::Mutex};
use alloc::{boxed::Box, collections::btree_map::BTreeMap, sync::Arc};
use core::fmt::Debug;

#[derive(Debug)]
pub struct FileSystem {
    name: &'static str,
    ops: Box<dyn FileSystemOps>,
}

impl FileSystem {
    pub fn new(name: &'static str, ops: Box<dyn FileSystemOps>) -> Self {
        Self { name, ops }
    }
}

pub trait FileSystemOps: Debug {
    fn mount(&self, fs: FileSystem, mount_point: Arc<Entry>);

    /// Gets the status of the file system.
    fn statvfs(&self) -> EResult<uapi::statvfs>;

    /// Synchronizes the entire file system.
    fn sync(&self) -> EResult<()>;
}

/// A super block is the overarching control structure of a file system.
/// It provides operations to create, modify and delete inodes.
#[derive(Debug)]
pub struct SuperBlock {}

pub trait SuperBlockOps {
    /// Allocates a new inode on this superblock.
    fn create_inode(&self, sb: &SuperBlock) -> Arc<INode>;
    /// Deletes the inode.
    fn destroy_inode(&self, inode: Arc<INode>);
    /// Marks this inode as dirty.
    fn dirty_inode(&self, inode: Arc<INode>);
}

/// A map of all known and registered file systems.
static FS_TABLE: Mutex<BTreeMap<&'static str, FileSystem>> = Mutex::new(BTreeMap::new());
