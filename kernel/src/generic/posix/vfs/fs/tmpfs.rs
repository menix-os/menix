use super::{FileSystem, SuperBlock};
use crate::generic::{
    memory::{VirtAddr, virt::AddressSpace},
    posix::{
        errno::{EResult, Errno},
        vfs::{
            entry::Entry,
            file::{File, FileOps},
            fs::MountFlags,
            inode::{INode, NodeOps},
            path::PathBuf,
        },
    },
    util::mutex::Mutex,
};
use alloc::{
    boxed::Box,
    sync::{Arc, Weak},
};
use uapi::{S_IFDIR, S_IRGRP, S_IROTH, S_IRUSR};

#[derive(Debug)]
pub struct TmpFs;
impl FileSystem for TmpFs {
    fn get_name(&self) -> &'static str {
        "tmpfs"
    }

    fn mount(
        &self,
        mount_point: PathBuf,
        flags: MountFlags,
    ) -> EResult<(Arc<dyn SuperBlock>, Arc<INode>)> {
        let sb = Arc::new(TmpFsSuper {
            root: Mutex::new(Weak::new()),
            flags,
        });

        let root = Arc::new(INode::new(
            Box::new(TmpFsNode {}),
            Box::new(TmpFsFile {}),
            sb.clone(),
            S_IFDIR | S_IRUSR | S_IRGRP | S_IROTH,
        )?);

        *sb.root.lock() = Arc::downgrade(&root);

        // TODO: Register at mount point.

        return Ok((sb, root));
    }
}

#[derive(Debug)]
pub struct TmpFsSuper {
    // TODO: Is this right?
    root: Mutex<Weak<INode>>,
    flags: MountFlags,
}

impl SuperBlock for TmpFsSuper {
    fn unmount(self) -> EResult<()> {
        Ok(())
    }

    fn statvfs(self: Arc<Self>) -> EResult<uapi::statvfs> {
        todo!()
    }

    fn sync(self: Arc<Self>) -> EResult<()> {
        // The entire FS is in memory, nothing to sync.
        Ok(())
    }

    fn create_inode(self: Arc<Self>, mode: uapi::mode_t) -> EResult<Arc<INode>> {
        let result = INode::new(
            Box::new(TmpFsNode {}),
            Box::new(TmpFsFile {}),
            self.clone(),
            mode,
        )?;
        return Ok(Arc::new(result));
    }

    fn destroy_inode(self: Arc<Self>, inode: INode) -> EResult<()> {
        todo!()
    }

    fn get_root(self: Arc<Self>) -> EResult<Arc<INode>> {
        return self.root.lock().upgrade().ok_or(Errno::ENOENT);
    }
}

#[derive(Debug)]
struct TmpFsNode {}
impl NodeOps for TmpFsNode {
    fn lookup(&self, entry: &Entry, directory: &INode) -> EResult<()> {
        todo!()
    }

    fn sync(&self, _: &INode) -> EResult<()> {
        // Every node is in memory, nothing to sync.
        Ok(())
    }

    fn read_symlink(&self, node: &INode, out: &mut [u8]) -> EResult<usize> {
        todo!()
    }
}

#[derive(Debug)]
struct TmpFsFile {}
impl FileOps for TmpFsFile {
    fn read(&self, file: &File, buffer: &mut [u8]) -> EResult<usize> {
        todo!()
    }

    fn write(&self, file: &File, buffer: &[u8]) -> EResult<usize> {
        todo!()
    }

    fn seek(&self, file: &File, offset: isize, whence: isize) -> EResult<usize> {
        todo!()
    }

    fn ioctl(&self, file: &File, request: usize, arg: usize) -> EResult<usize> {
        todo!()
    }

    fn poll(&self, file: &File, mask: u32) -> EResult<usize> {
        todo!()
    }

    fn mmap(
        &self,
        file: &File,
        space: &AddressSpace,
        offset: u64,
        hint: usize,
        size: usize,
    ) -> EResult<VirtAddr> {
        todo!()
    }
}
