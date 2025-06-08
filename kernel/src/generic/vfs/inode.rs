use super::fs::SuperBlock;
use crate::generic::{
    posix::errno::EResult,
    util::mutex::Mutex,
    vfs::{entry::Entry, file::FileOps},
};
use alloc::{boxed::Box, sync::Arc};
use core::{fmt::Debug, sync::atomic::AtomicBool};

/// A standalone inode. See [`super::entry::Entry`] for information.
#[derive(Debug)]
pub struct INode {
    pub id: u64,

    /// FS-specific callbacks that can be performed on this node.
    pub node_ops: Box<dyn NodeOps>,
    pub file_ops: Box<dyn FileOps>,

    /// The super block which this node is located in.
    pub sb: Arc<dyn SuperBlock>,

    /// If true, the node has been modified and has to be sync'd.
    pub dirty: AtomicBool,

    pub stat: Mutex<Stat>,
}

/// Operations which work on a node.
pub trait NodeOps: Debug {
    /// Updates the node with given timestamps. If one of the arguments is [`None`], it is not updated.
    fn update_time(
        &self,
        node: &INode,
        mtime: Option<uapi::timespec>,
        atime: Option<uapi::timespec>,
        ctime: Option<uapi::timespec>,
    ) -> EResult<()>;

    /// Attempts to resolve an `entry` in a given `node` directory.
    /// If a node is found, the target node is set on `entry`.
    /// If it isn't found, the entry is marked negative and [`Errno::ENOENT`] is returned.
    fn lookup(&self, node: &INode, entry: &Entry) -> EResult<()>;

    /// Synchronizes the node back to the underlying file system.
    fn sync(&self, node: &INode) -> EResult<()>;

    /// Reads the path of the symbolic link of the node into a buffer.
    fn readlink(&self, node: &INode, out: &mut [u8]) -> EResult<usize>;
}

#[derive(Debug, Clone, Default)]
pub enum NodeType {
    #[default]
    Regular,
    Directory,
    SymbolicLink,
    FIFO,
    BlockDevice,
    CharacterDevice,
    Socket,
}

#[derive(Debug, Default)]
pub struct Stat {
    pub links: u64,
    pub size: u64,
    pub blocks: u64,
    pub node_type: NodeType,
    pub mode: u32,
}
