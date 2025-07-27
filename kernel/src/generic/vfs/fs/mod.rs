pub mod devtmpfs;
pub mod initramfs;
mod tmpfs;

use super::inode::INode;
use crate::generic::{
    posix::errno::{EResult, Errno},
    util::spin_mutex::SpinMutex,
    vfs::{
        PathNode,
        cache::Entry,
        inode::{Mode, NodeType},
    },
};
use alloc::{collections::btree_map::BTreeMap, string::String, sync::Arc};
use core::fmt::Debug;

/// A mounted file system.
#[derive(Debug)]
pub struct Mount {
    pub flags: MountFlags,
    pub super_block: Arc<dyn SuperBlock>,
    pub root: Arc<Entry>,
    pub mount_point: SpinMutex<Option<PathNode>>,
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct MountFlags: u32 {
        const ReadOnly = uapi::MS_RDONLY as _;
        const NoSuperUserID = uapi::MS_NOSUID as _;
        const NoDev = uapi::MS_NODEV as _;
        const NoExec = uapi::MS_NOEXEC as _;
        const NoSynchronous = uapi::MS_SYNCHRONOUS as _;
        const Remount = uapi::MS_REMOUNT as _;
        const MandatoryLock = uapi::MS_MANDLOCK as _;
        const DirSync = uapi::MS_DIRSYNC as _;
        const NoSymbolFollow = uapi::MS_NOSYMFOLLOW as _;
        const NoAccessTime = uapi::MS_NOATIME as _;
        const NoDirAccessTime = uapi::MS_NODIRATIME as _;
        const Bind = uapi::MS_BIND as _;
        const Move = uapi::MS_MOVE as _;
        const Rec = uapi::MS_REC as _;
        const Silent = uapi::MS_SILENT as _;
        const PosixACL = uapi::MS_POSIXACL as _;
        const Unbindable = uapi::MS_UNBINDABLE as _;
        const Private = uapi::MS_PRIVATE as _;
        const Slave = uapi::MS_SLAVE as _;
        const Shared = uapi::MS_SHARED as _;
        const RelativeTime = uapi::MS_RELATIME as _;
        const KernMount = uapi::MS_KERNMOUNT as _;
        const IVersion = uapi::MS_I_VERSION as _;
        const StrictAccessTime = uapi::MS_STRICTATIME as _;
        const LazyTime = uapi::MS_LAZYTIME as _;
        const NoRemoteLock = uapi::MS_NOREMOTELOCK as _;
        const NoSec = uapi::MS_NOSEC as _;
        const Born = uapi::MS_BORN as _;
        const Active = uapi::MS_ACTIVE as _;
        const NoUser = uapi::MS_NOUSER as _;
    }
}

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
    fn create_inode(self: Arc<Self>, node_type: NodeType, mode: Mode) -> EResult<Arc<INode>>;

    /// Deletes the inode.
    fn destroy_inode(self: Arc<Self>, inode: INode) -> EResult<()>;
}

/// A map of all known and registered file systems.
static FS_TABLE: SpinMutex<BTreeMap<&'static [u8], &'static dyn FileSystem>> =
    SpinMutex::new(BTreeMap::new());

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
