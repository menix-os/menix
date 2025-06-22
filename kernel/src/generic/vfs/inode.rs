use super::fs::SuperBlock;
use crate::generic::{
    posix::errno::{EResult, Errno},
    process::Identity,
    vfs::{
        PathNode,
        cache::Entry,
        file::{File, FileOps, OpenFlags},
    },
};
use alloc::{boxed::Box, sync::Arc};
use core::{fmt::Debug, sync::atomic::AtomicBool};

/// A standalone inode. See [`super::cache::Entry`] for information.
#[derive(Debug)]
pub struct INode {
    pub id: u64,
    /// Operations that work on every type of node.
    pub common_ops: Box<dyn CommonOps>,
    /// Operations that only work on a certain type of node.
    pub node_ops: NodeOps,
    /// Operations that can be performed on an open file pointing to this node.
    pub file_ops: Arc<dyn FileOps>,
    /// The super block which this node is located in.
    pub sb: Arc<dyn SuperBlock>,
    /// If true, the node has been modified and has to be sync'd.
    pub dirty: AtomicBool,
}

impl INode {
    /// Checks if the node can be accessed with the given identity.
    /// Returns [`Errno::EACCES`] if an access is not allowed.
    pub fn try_access(&self, ident: &Identity, flags: OpenFlags, use_real: bool) -> EResult<()> {
        if ident.effective_user_id == 0 {
            // If this file is not able to be executed, always fail.
            if flags.contains(OpenFlags::Executeable)
                && !self
                    .common_ops
                    .get_mode()?
                    .contains(Mode::UserExec | Mode::GroupExec | Mode::OtherExec)
            {
                return Err(Errno::EACCES);
            }
            return Ok(());
        }

        todo!()
    }
}

/// Operations which work on any kind of [`INode`].
pub trait CommonOps: Debug {
    /// Updates the node with given timestamps.
    /// If an argument is [`None`], the respective value is not updated.
    fn update_time(
        &self,
        node: &INode,
        mtime: Option<uapi::timespec>,
        atime: Option<uapi::timespec>,
        ctime: Option<uapi::timespec>,
    ) -> EResult<()>;

    fn get_mode(&self) -> EResult<Mode>;

    /// Changes permissions on this `node`.
    fn chmod(&self, node: &INode, mode: Mode) -> EResult<()>;

    /// Changes ownership on this `node`.
    fn chown(&self, node: &INode, uid: uapi::uid_t, gid: uapi::gid_t) -> EResult<()>;

    /// Synchronizes the node back to the underlying file system.
    fn sync(&self, node: &INode) -> EResult<()>;
}

#[derive(Debug)]
pub enum NodeOps {
    Regular(Box<dyn RegularOps>),
    Directory(Box<dyn DirectoryOps>),
    SymbolicLink(Box<dyn SymlinkOps>),
    FIFO,
    BlockDevice,
    CharacterDevice,
    Socket,
}

/// Operations for directory [`INode`]s.
pub trait DirectoryOps: Debug {
    fn open(&self, node: &Arc<INode>, entry: PathNode, flags: OpenFlags) -> EResult<Arc<File>>;

    /// Looks up all children in an `node` directory and caches them in `entry`.
    fn lookup(&self, node: &Arc<INode>, entry: &mut Entry) -> EResult<()>;
}

/// Operations for regular file [`INode`]s.
pub trait RegularOps: Debug {
    /// Truncates the node to a given length in bytes.
    /// `length` must be equal or less than the current node size.
    fn truncate(&self, node: &INode, length: u64) -> EResult<()>;

    fn read(&self, node: &INode, buf: &mut [u8], offset: u64) -> EResult<u64>;
    fn write(&self, node: &INode, buf: &[u8], offset: u64) -> EResult<u64>;
}

/// Operations for symbolic link [`INode`]s.
pub trait SymlinkOps: Debug {
    /// Reads the path of the symbolic link of the node.
    fn read_link(&self, node: &INode) -> EResult<usize>;
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
