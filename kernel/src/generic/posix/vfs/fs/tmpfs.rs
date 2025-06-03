use super::{FileSystem, SuperBlock};
use crate::generic::posix::{
    errno::EResult,
    vfs::{
        entry::Entry,
        inode::{INode, NodeOps},
    },
};
use alloc::{sync::Arc, vec::Vec};

#[derive(Debug)]
pub struct TmpFs;
impl FileSystem for TmpFs {
    fn get_name(&self) -> &'static str {
        "tmpfs"
    }

    fn mount(&self, mount_point: Arc<Entry>) -> EResult<(Arc<dyn SuperBlock>, Arc<INode>)> {
        let mut sb = TmpFsSuper { root: None };
        let root = sb.create_inode(uapi::S_IRUSR | uapi::S_IRGRP | uapi::S_IROTH)?;
        sb.root = Some(root.clone());

        return Ok((Arc::new(sb), root));
    }
}

#[derive(Debug)]
pub struct TmpFsSuper {
    root: Option<Arc<INode>>,
}

impl SuperBlock for TmpFsSuper {
    fn unmount(&self) -> EResult<()> {
        todo!()
    }

    fn statvfs(&self) -> EResult<uapi::statvfs> {
        todo!()
    }

    fn sync(&self) -> EResult<()> {
        todo!()
    }

    fn create_inode(&self, mode: uapi::mode_t) -> EResult<Arc<INode>> {
        todo!()
    }

    fn destroy_inode(&self, inode: INode) -> EResult<()> {
        todo!()
    }
}

#[derive(Debug)]
struct TmpFsOps;
impl NodeOps for TmpFsOps {
    fn lookup(&self, entry: &Entry, directory: &INode) -> EResult<()> {
        todo!()
    }

    fn sync(&self, node: &INode) -> EResult<()> {
        todo!()
    }

    fn read_symlink(&self, node: &INode, out: &mut Vec<u8>) -> EResult<usize> {
        todo!()
    }
}
