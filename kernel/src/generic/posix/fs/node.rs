use super::FileSystem;
use crate::generic::posix::errno::EResult;
use alloc::sync::Arc;

/// A virtual file system node.
#[derive(Debug)]
pub struct Node {
    fs: Arc<FileSystem>,
}

pub trait NodeOps {
    fn lookup() -> EResult<()>;
}

impl Node {
    pub fn new() -> EResult<Arc<Self>> {
        todo!()
    }
}
