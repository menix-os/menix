#![allow(unused)]

use super::{MountFlags, SuperBlock};
use crate::generic::{
    memory::{VirtAddr, virt::AddressSpace},
    posix::errno::{EResult, Errno},
    util::mutex::Mutex,
    vfs::{
        PathNode,
        cache::Entry,
        file::{File, FileOps, OpenFlags, SeekAnchor},
        fs::{FileSystem, Mount},
        inode::{CommonOps, DirectoryOps, INode, Mode, NodeOps, NodeType, RegularOps},
    },
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

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
        let node_ops = match node_type {
            NodeType::Regular => NodeOps::Regular(Box::new(TmpRegular::default())),
            NodeType::Directory => NodeOps::Directory(Box::new(TmpDir::default())),
            _ => return Err(Errno::EINVAL),
        };

        let node = INode {
            id: self.inode_counter.fetch_add(1, Ordering::Acquire) as u64,
            common_ops: Box::try_new(TmpNode {
                mode,
                ..Default::default()
            })?,
            node_ops,
            file_ops: Arc::try_new(TmpFile::default())?,
            sb: self,
            dirty: AtomicBool::new(false),
        };

        return Ok(Arc::try_new(node)?);
    }

    fn destroy_inode(self: Arc<Self>, inode: INode) -> EResult<()> {
        todo!()
    }
}

#[derive(Debug, Default)]
struct TmpNode {
    mtime: Mutex<uapi::timespec>,
    atime: Mutex<uapi::timespec>,
    ctime: Mutex<uapi::timespec>,
    mode: Mode,
}

impl CommonOps for TmpNode {
    fn update_time(
        &self,
        _node: &INode,
        mtime: Option<uapi::timespec>,
        atime: Option<uapi::timespec>,
        ctime: Option<uapi::timespec>,
    ) -> EResult<()> {
        if let Some(x) = mtime {
            *self.mtime.lock() = x;
        }
        if let Some(x) = atime {
            *self.atime.lock() = x;
        }
        if let Some(x) = ctime {
            *self.ctime.lock() = x;
        }
        Ok(())
    }

    fn chmod(&self, node: &INode, mode: Mode) -> EResult<()> {
        todo!()
    }

    fn chown(&self, node: &INode, uid: uapi::uid_t, gid: uapi::gid_t) -> EResult<()> {
        todo!()
    }

    fn sync(&self, _node: &INode) -> EResult<()> {
        // This is a no-op.
        Ok(())
    }

    fn get_mode(&self) -> EResult<Mode> {
        Ok(self.mode.clone())
    }
}

#[derive(Debug, Default)]
struct TmpDir {}

impl DirectoryOps for TmpDir {
    fn open(&self, node: &Arc<INode>, path: PathNode, flags: OpenFlags) -> EResult<Arc<File>> {
        let file = File {
            path: Some(path),
            ops: Arc::new(TmpFile::default()),
            inode: Some(node.clone()),
            flags,
            position: AtomicUsize::new(0),
        };
        return Ok(Arc::try_new(file)?);
    }

    fn lookup(&self, node: &Arc<INode>, entry: &mut Entry) -> EResult<()> {
        // tmpfs directories only live in memory, so we cannot look up entries that do not exist.
        return Err(Errno::ENOENT);
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

    fn seek(&self, file: &File, offset: SeekAnchor) -> EResult<u64> {
        todo!()
    }

    fn ioctl(&self, file: &File, request: usize, arg: usize) -> EResult<usize> {
        todo!()
    }

    fn mmap(
        &self,
        file: &File,
        space: &AddressSpace,
        offset: u64,
        hint: VirtAddr,
        size: usize,
    ) -> EResult<VirtAddr> {
        todo!()
    }
}

init_stage! {
    #[depends(crate::generic::memory::MEMORY_STAGE)]
    #[entails(crate::generic::vfs::VFS_STAGE)]
    TMPFS_INIT: "generic.vfs.tmpfs" => || super::register_fs(&TmpFs);
}
