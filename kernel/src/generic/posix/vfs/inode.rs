use super::{fs::SuperBlock, path::PathBuf};
use crate::generic::{posix::errno::EResult, util::mutex::Mutex};
use alloc::{boxed::Box, sync::Arc};
use core::{fmt::Debug, sync::atomic::AtomicBool};

/// A standalone inode. See [`super::entry::Entry`] for information.
#[derive(Debug)]
pub struct INode {
    /// FS-specific callbacks that can be performed on this node.
    pub ops: Box<dyn NodeOps>,
    /// The super block which this node is located in.
    pub sb: Arc<SuperBlock>,
    /// The status of this node.
    pub stat: Mutex<uapi::stat>,
    /// If true, the node has been modified and has to be sync'd.
    pub dirty: AtomicBool,
}

/// Operations which work on a node.
pub trait NodeOps: Debug {
    /// Looks up an entry in `this` directory.
    fn lookup(&self, this: &INode) -> EResult<()>;

    /// Synchronizes this node back to the underlying file system.
    fn sync(&self, this: &INode) -> EResult<()>;
}

impl INode {
    pub fn get_stat(&self) -> uapi::stat {
        self.stat.lock().clone()
    }

    pub fn read_link(&self) -> EResult<PathBuf> {
        todo!()
    }
}
