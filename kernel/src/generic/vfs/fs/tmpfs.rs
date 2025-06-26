#![allow(unused)]

use super::{MountFlags, SuperBlock};
use crate::generic::{
    memory::{
        VirtAddr,
        cache::PageCache,
        virt::{VmRegion, VmSpace},
    },
    posix::errno::{EResult, Errno},
    process::Identity,
    util::mutex::Mutex,
    vfs::{
        PathNode,
        cache::Entry,
        file::{File, FileOps, OpenFlags, SeekAnchor},
        fs::{FileSystem, Mount},
        inode::{CommonOps, DirectoryOps, INode, Mode, NodeOps, NodeType, RegularOps, SymlinkOps},
    },
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{
    any::Any,
    sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering},
};

#[derive(Debug)]
struct TmpFs;

impl FileSystem for TmpFs {
    fn get_name(&self) -> &'static [u8] {
        b"tmpfs"
    }

    fn mount(&self, _: Option<Arc<Entry>>, flags: MountFlags) -> EResult<Arc<Mount>> {
        let super_block = Arc::try_new(TmpSuper {
            inode_counter: AtomicUsize::new(0),
        })?;

        let root_inode = super_block
            .clone()
            .create_inode(NodeType::Directory, Mode::from_bits_truncate(0o755))?;

        Ok(Arc::try_new(Mount {
            flags,
            super_block,
            root: Arc::try_new(Entry::new(b"", Some(root_inode), None))?,
            mount_point: Mutex::default(),
        })?)
    }
}

#[derive(Debug)]
struct TmpSuper {
    inode_counter: AtomicUsize,
}

impl SuperBlock for TmpSuper {
    fn sync(self: Arc<Self>) -> EResult<()> {
        // This is a no-op.
        Ok(())
    }

    fn statvfs(self: Arc<Self>) -> EResult<uapi::statvfs> {
        todo!()
    }

    fn create_inode(self: Arc<Self>, node_type: NodeType, mode: Mode) -> EResult<Arc<INode>> {
        let node = INode {
            id: self.inode_counter.fetch_add(1, Ordering::Acquire) as u64,
            common_ops: Box::try_new(TmpNode)?,
            node_ops: match node_type {
                NodeType::Regular => NodeOps::Regular(Box::new(TmpRegular::default())),
                NodeType::SymbolicLink => NodeOps::SymbolicLink(Box::new(TmpRegular::default())),
                NodeType::Directory => NodeOps::Directory(Box::new(TmpDir::default())),
                _ => return Err(Errno::EINVAL),
            },
            file_ops: Arc::try_new(TmpFile::default())?,
            sb: self,
            mode: AtomicU32::new(mode.bits()),
            cache: PageCache::default(),
            atime: Mutex::default(),
            mtime: Mutex::default(),
            ctime: Mutex::default(),
            size: AtomicU64::default(),
            uid: AtomicUsize::default(),
            gid: AtomicUsize::default(),
        };

        return Ok(Arc::try_new(node)?);
    }

    fn destroy_inode(self: Arc<Self>, inode: INode) -> EResult<()> {
        match Arc::into_inner(self) {
            Some(x) => {
                drop(x);
                Ok(())
            }
            None => Err(Errno::EBUSY),
        }
    }
}

#[derive(Debug, Default)]
struct TmpNode;

impl CommonOps for TmpNode {
    fn sync(&self, _node: &INode) -> EResult<()> {
        // This is a no-op.
        Ok(())
    }

    fn sync_page(&self, node: &INode, page: &crate::generic::memory::pmm::Page) -> EResult<()> {
        // This is a no-op.
        Ok(())
    }
}

#[derive(Debug, Default)]
struct TmpDir {}

impl DirectoryOps for TmpDir {
    fn lookup(&self, node: &Arc<INode>, entry: &PathNode) -> EResult<()> {
        // tmpfs directories only live in memory, so we cannot look up entries that do not exist.
        return Err(Errno::ENOENT);
    }

    fn open(
        &self,
        node: &Arc<INode>,
        path: PathNode,
        flags: OpenFlags,
        identity: &Identity,
    ) -> EResult<Arc<File>> {
        let file = File {
            path: Some(path),
            ops: Arc::new(TmpFile::default()),
            inode: Some(node.clone()),
            flags,
            position: AtomicU64::new(0),
        };
        return Ok(Arc::try_new(file)?);
    }

    fn symlink(
        &self,
        node: &Arc<INode>,
        path: PathNode,
        target_path: &[u8],
        identity: &Identity,
    ) -> EResult<()> {
        let sym_inode = node
            .sb
            .clone()
            .create_inode(NodeType::SymbolicLink, Mode::from_bits_truncate(0o777))?;

        match &sym_inode.node_ops {
            NodeOps::SymbolicLink(x) => {
                let data: &TmpRegular = (x.as_ref() as &dyn Any)
                    .downcast_ref()
                    .ok_or(Errno::EINVAL)?;
                data.data.lock().extend_from_slice(target_path);
                path.entry.set_inode(sym_inode);

                Ok(())
            }
            _ => Err(Errno::EINVAL),
        }
    }

    fn link(&self, node: &Arc<INode>, path: &PathNode, target: &Arc<INode>) -> EResult<()> {
        path.entry.set_inode(target.clone());
        Ok(())
    }

    fn unlink(&self, node: &Arc<INode>, entry: &PathNode) -> EResult<()> {
        todo!()
    }

    fn rename(
        &self,
        node: &Arc<INode>,
        entry: PathNode,
        target: &Arc<INode>,
        target_entry: PathNode,
    ) -> EResult<()> {
        todo!()
    }
}

#[derive(Debug, Default)]
struct TmpRegular {
    data: Mutex<Vec<u8>>,
}

impl RegularOps for TmpRegular {
    fn truncate(&self, node: &INode, length: u64) -> EResult<()> {
        self.data.lock().truncate(length as usize);
        Ok(())
    }

    fn read(&self, node: &INode, buf: &mut [u8], offset: u64) -> EResult<u64> {
        let mut v = self.data.lock();
        if offset as usize >= v.len() {
            return Ok(0);
        }

        let copy_size = buf.len().min(v.len() - offset as usize);
        buf.copy_from_slice(&v[offset as usize..][..copy_size]);

        Ok(copy_size as u64)
    }

    fn write(&self, node: &INode, buf: &[u8], offset: u64) -> EResult<u64> {
        let mut v = self.data.lock();
        if offset as usize + buf.len() >= v.len() {
            v.resize(offset as usize + buf.len(), 0u8);
        }
        v[offset as usize..][..buf.len()].copy_from_slice(buf);

        Ok(buf.len() as u64)
    }
}

impl SymlinkOps for TmpRegular {
    fn read_link(&self, node: &INode, buf: &mut [u8]) -> EResult<u64> {
        let mut v = self.data.lock();
        let copy_size = buf.len().min(v.len());
        &buf[0..copy_size].copy_from_slice(&v[0..copy_size]);
        Ok(copy_size as u64)
    }
}

#[derive(Debug, Default)]
struct TmpFile {
    length: AtomicUsize,
}

impl FileOps for TmpFile {
    fn read_dir(&self, file: &File, buffer: &mut [u8]) -> EResult<u64> {
        todo!()
    }

    fn read(&self, file: &File, buffer: &mut [u8], offset: u64) -> EResult<u64> {
        let inode = file.inode.as_ref().unwrap();

        match &inode.node_ops {
            NodeOps::Regular(regular_ops) => regular_ops.read(inode, buffer, offset),
            _ => todo!(),
        }
    }

    fn write(&self, file: &File, buffer: &[u8], offset: u64) -> EResult<u64> {
        let inode = file.inode.as_ref().unwrap();

        match &inode.node_ops {
            NodeOps::Regular(regular_ops) => regular_ops.write(inode, buffer, offset),
            _ => todo!(),
        }
    }

    fn mmap(
        &self,
        file: &File,
        space: &VmSpace,
        offset: u64,
        hint: VirtAddr,
        size: usize,
    ) -> EResult<VirtAddr> {
        todo!()
    }

    fn poll(&self, file: &File, mask: u16) -> EResult<u16> {
        todo!()
    }
}

init_stage! {
    #[depends(crate::generic::memory::MEMORY_STAGE)]
    #[entails(crate::generic::vfs::VFS_STAGE)]
    TMPFS_INIT: "generic.vfs.tmpfs" => || super::register_fs(&TmpFs);
}
