#![allow(unused)]

use super::inode::INode;
use crate::generic::{
    util::mutex::Mutex,
    vfs::{fs::SuperBlock, path::Path},
};
use alloc::{
    collections::btree_map::BTreeMap,
    sync::{Arc, Weak},
    vec::Vec,
};

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct EntryFlags: u32 {
        const Cached = 0;
    }
}

/// This struct represents an entry in the VFS.
#[derive(Debug)]
pub struct Entry {
    /// The name of this entry.
    pub name: Vec<u8>,
    flags: EntryFlags,
    /// The underlying [`INode`] this entry is pointing to.
    /// A [`None`] value indicates that this entry is negative.
    inode: Mutex<Option<Weak<INode>>>,
    /// The parent of this [`Entry`].
    /// A [`None`] value indicates that this entry is a root.
    parent: Option<Arc<Entry>>,
    /// If the [`EntryFlags::Cached`] bit is set in [`Self::flags`],
    /// then this contains a map of all children of this entry.
    children: Mutex<BTreeMap<Vec<u8>, Weak<Entry>>>,
    /// A list of mounts on this entry.
    mounts: Mutex<Vec<Weak<Mount>>>,
}

impl Entry {
    pub fn new(name: &[u8], inode: Option<Weak<INode>>, parent: Option<Arc<Entry>>) -> Self {
        Entry {
            name: name.to_vec(),
            flags: EntryFlags::empty(),
            inode: Mutex::new(inode),
            parent,
            children: Mutex::new(BTreeMap::new()),
            mounts: Mutex::new(Vec::new()),
        }
    }

    pub fn get_inode(&self) -> Option<Arc<INode>> {
        if self.flags.contains(EntryFlags::Cached) {
            return self.inode.lock().as_ref()?.upgrade();
        }

        // Do lookup if it wasn't cached already.
        todo!()
    }
}

/// A mounted file system.
#[derive(Debug)]
pub struct Mount {
    pub flags: MountFlags,
    pub super_block: Arc<dyn SuperBlock>,
    pub root: Arc<Entry>,
    pub mount_point: Mutex<Option<Path>>,
}

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
