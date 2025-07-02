use super::fs::SuperBlock;
use crate::generic::{
    device::{BlockDevice, CharDevice},
    memory::cache::{Object, Pager},
    posix::errno::{EResult, Errno},
    process::Identity,
    util::mutex::Mutex,
    vfs::{
        PathNode,
        file::{File, FileOps, OpenFlags},
    },
};
use alloc::{boxed::Box, sync::Arc};
use core::{
    any::Any,
    fmt::Debug,
    sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering},
};

/// A standalone inode. See [`super::cache::Entry`] for information.
#[derive(Debug)]
pub struct INode {
    /// Operations that work on every type of node.
    pub common_ops: Box<dyn CommonOps>,
    /// Operations that only work on a certain type of node.
    pub node_ops: NodeOps,
    /// Operations that can be performed on an open file pointing to this node.
    pub file_ops: Arc<dyn FileOps>,
    /// The super block which this node is located in.
    pub sb: Arc<dyn SuperBlock>,
    pub object: Object,

    // The following fields make up `stat`.
    pub id: u64,
    pub size: AtomicU64,
    pub uid: AtomicUsize,
    pub gid: AtomicUsize,
    pub atime: Mutex<uapi::timespec>,
    pub mtime: Mutex<uapi::timespec>,
    pub ctime: Mutex<uapi::timespec>,
    pub mode: AtomicU32,
}

impl INode {
    /// Checks if the node can be accessed with the given identity.
    /// Returns [`Errno::EACCES`] if an access is not allowed.
    pub fn try_access(&self, ident: &Identity, flags: OpenFlags, use_real: bool) -> EResult<()> {
        if ident.effective_user_id == 0 {
            // If this file is not able to be executed, always fail.
            if flags.contains(OpenFlags::Executeable)
                && !Mode::from_bits_truncate(self.mode.load(Ordering::Acquire))
                    .contains(Mode::UserExec | Mode::GroupExec | Mode::OtherExec)
            {
                return Err(Errno::EACCES);
            }
            return Ok(());
        }

        todo!("Implement UID handling for !root");
    }

    /// Updates the node with given timestamps.
    /// If an argument is [`None`], the respective value is not updated.
    pub fn update_time(
        &self,
        node: &INode,
        mtime: Option<uapi::timespec>,
        atime: Option<uapi::timespec>,
        ctime: Option<uapi::timespec>,
    ) -> EResult<()> {
        todo!();
    }

    /// Returns the current mode of this inode.
    pub fn get_mode(&self) -> Mode {
        Mode::from_bits_truncate(self.mode.load(Ordering::Acquire))
    }

    /// Changes permissions on this `node`.
    pub fn chmod(&self, node: &INode, mode: Mode) -> EResult<()> {
        todo!();
    }

    /// Changes ownership on this `node`.
    pub fn chown(&self, node: &INode, uid: uapi::uid_t, gid: uapi::gid_t) -> EResult<()> {
        todo!();
    }
}

impl Pager for INode {
    fn get_pages(
        &self,
        object: &Object,
        pages: &[usize],
        faulty_page: usize,
    ) -> Result<&[usize], crate::generic::memory::cache::PagerError> {
        todo!()
    }

    fn write_pages(
        &self,
        object: &Object,
        pages: &[u64],
    ) -> Result<(), crate::generic::memory::cache::PagerError> {
        todo!()
    }
}

/// Operations which work on any kind of [`INode`].
pub trait CommonOps: Debug {
    /// Synchronizes the node metadata back to the underlying file system.
    fn sync(&self, node: &INode) -> EResult<()>;
}

#[derive(Debug)]
pub enum NodeOps {
    Regular(Box<dyn RegularOps>),
    Directory(Box<dyn DirectoryOps>),
    SymbolicLink(Box<dyn SymlinkOps>),
    FIFO,
    BlockDevice(Arc<BlockDevice>),
    CharacterDevice(Arc<CharDevice>),
    Socket,
}

/// Operations for directory [`INode`]s.
pub trait DirectoryOps: Any + Debug {
    /// Looks up all children in an `node` directory and caches them in `entry`.
    /// An implementation shall return [`Errno::ENOENT`] if a lookup fails and
    /// shall leave `entry` unchanged.
    fn lookup(&self, node: &Arc<INode>, entry: &PathNode) -> EResult<()>;

    /// Opens a directory.
    fn open(
        &self,
        node: &Arc<INode>,
        entry: PathNode,
        flags: OpenFlags,
        identity: &Identity,
    ) -> EResult<Arc<File>>;

    /// Creates a new symbolic link.
    fn symlink(
        &self,
        node: &Arc<INode>,
        entry: PathNode,
        target_path: &[u8],
        identity: &Identity,
    ) -> EResult<()>;

    /// Creates a new hard link.
    fn link(&self, node: &Arc<INode>, entry: &PathNode, target: &Arc<INode>) -> EResult<()>;

    /// Removes a link.
    fn unlink(&self, node: &Arc<INode>, entry: &PathNode) -> EResult<()>;

    /// Renames a node.
    fn rename(
        &self,
        node: &Arc<INode>,
        entry: PathNode,
        target: &Arc<INode>,
        target_entry: PathNode,
    ) -> EResult<()>;
}

/// Operations for regular file [`INode`]s.
pub trait RegularOps: Any + Debug {
    /// Truncates the node to a given length in bytes.
    /// `length` must be equal or less than the current node size.
    fn truncate(&self, node: &INode, length: u64) -> EResult<()>;

    fn read(&self, node: &INode, buf: &mut [u8], offset: u64) -> EResult<u64>;
    fn write(&self, node: &INode, buf: &[u8], offset: u64) -> EResult<u64>;
}

/// Operations for symbolic link [`INode`]s.
pub trait SymlinkOps: Any + Debug {
    /// Reads the path of the symbolic link of the node.
    /// Returns amount of bytes read into the buffer.
    fn read_link(&self, node: &INode, buf: &mut [u8]) -> EResult<u64>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Regular = uapi::S_IFREG as _,
    Directory = uapi::S_IFDIR as _,
    SymbolicLink = uapi::S_IFLNK as _,
    FIFO = uapi::S_IFIFO as _,
    BlockDevice = uapi::S_IFBLK as _,
    CharacterDevice = uapi::S_IFCHR as _,
    Socket = uapi::S_IFSOCK as _,
}

bitflags::bitflags! {
    #[derive(Debug, Default, Clone)]
    pub struct Mode: u32 {
        const UserRead = uapi::S_IRUSR;
        const UserWrite = uapi::S_IWUSR;
        const UserExec = uapi::S_IXUSR;

        const GroupRead = uapi::S_IRGRP;
        const GroupWrite = uapi::S_IWGRP;
        const GroupExec = uapi::S_IXGRP;

        const OtherRead = uapi::S_IROTH;
        const OtherWrite = uapi::S_IWOTH;
        const OtherExec = uapi::S_IXOTH;

        const SetUserId = uapi::S_ISUID;
        const SetGroupId = uapi::S_ISGID;
    }
}
