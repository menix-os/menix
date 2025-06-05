pub(super) mod initrd;
pub(super) mod tmpfs;

use super::inode::INode;
use crate::generic::{
    posix::{errno::EResult, vfs::path::PathBuf},
    util::mutex::Mutex,
};
use alloc::{boxed::Box, collections::btree_map::BTreeMap, sync::Arc};
use core::fmt::Debug;

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct MountFlags: u32 {
        const ReadOnly = uapi::MS_RDONLY as u32;
        const NoSuperUserID = uapi::MS_NOSUID as u32;
        const NoDev = uapi::MS_NODEV as u32;
        const NoExec = uapi::MS_NOEXEC as u32;
        const NoSynchronous = uapi::MS_SYNCHRONOUS as u32;
        const Remount = uapi::MS_REMOUNT as u32;
        const MandatoryLock = uapi::MS_MANDLOCK as u32;
        const DirSync = uapi::MS_DIRSYNC as u32;
        const NoSymbolFollow = uapi::MS_NOSYMFOLLOW as u32;
        const NoAccessTime = uapi::MS_NOATIME as u32;
        const NoDirAccessTime = uapi::MS_NODIRATIME as u32;
        const Bind = uapi::MS_BIND as u32;
        const Move = uapi::MS_MOVE as u32;
        const Rec = uapi::MS_REC as u32;
        const Silent = uapi::MS_SILENT as u32;
        const PosixACL = uapi::MS_POSIXACL as u32;
        const Unbindable = uapi::MS_UNBINDABLE as u32;
        const Private = uapi::MS_PRIVATE as u32;
        const Slave = uapi::MS_SLAVE as u32;
        const Shared = uapi::MS_SHARED as u32;
        const RelativeTime = uapi::MS_RELATIME as u32;
        const KernMount = uapi::MS_KERNMOUNT as u32;
        const IVersion = uapi::MS_I_VERSION as u32;
        const StrictAccessTime = uapi::MS_STRICTATIME as u32;
        const LazyTime = uapi::MS_LAZYTIME as u32;
        const NoRemoteLock = uapi::MS_NOREMOTELOCK as u32;
        const NoSec = uapi::MS_NOSEC as u32;
        const Born = uapi::MS_BORN as u32;
        const Active = uapi::MS_ACTIVE as u32;
        const NoUser = uapi::MS_NOUSER as u32;
    }
}

pub trait FileSystem: Debug {
    /// Returns an identifier which can be used to determine this file system.
    fn get_name(&self) -> &'static str;

    /// Mounts an instance of this file system on a `mount_point`.
    /// Returns a reference to the super block and the root of the file system.
    fn mount(
        &self,
        mount_point: PathBuf,
        flags: MountFlags,
    ) -> EResult<(Arc<dyn SuperBlock>, Arc<INode>)>;
}

/// A super block is the control structure of a file system instance.
/// It provides operations to create, modify and delete inodes.
pub trait SuperBlock: Debug {
    /// Unmounts this super block.
    fn unmount(self) -> EResult<()>;

    /// Gets the root node of this file system.
    fn get_root(self: Arc<Self>) -> EResult<Arc<INode>>;

    /// Synchronizes the entire file system.
    fn sync(self: Arc<Self>) -> EResult<()>;

    /// Gets the status of the file system.
    fn statvfs(self: Arc<Self>) -> EResult<uapi::statvfs>;

    /// Allocates a new inode on this super block.
    fn create_inode(self: Arc<Self>, mode: uapi::mode_t) -> EResult<Arc<INode>>;

    /// Deletes the inode.
    fn destroy_inode(self: Arc<Self>, inode: INode) -> EResult<()>;
}

/// Registers a new file system.
pub fn register_fs(fs: Box<dyn FileSystem>) {
    let name = fs.get_name();
    FS_TABLE.lock().insert(name, fs);
    log!("Registered new file system \"{}\"", name);
}

/// A map of all known and registered file systems.
static FS_TABLE: Mutex<BTreeMap<&'static str, Box<dyn FileSystem>>> = Mutex::new(BTreeMap::new());
