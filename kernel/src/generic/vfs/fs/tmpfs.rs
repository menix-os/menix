#![allow(unused)]

use super::{MountFlags, SuperBlock};
use crate::{
    arch,
    generic::{
        memory::{PhysAddr, cache::MemoryObject, virt::VmLevel},
        posix::errno::{EResult, Errno},
        process::Identity,
        util::spin_mutex::SpinMutex,
        vfs::{
            PathNode,
            cache::Entry,
            file::{File, FileOps, MmapFlags, OpenFlags, SeekAnchor},
            fs::{FileSystem, Mount},
            inode::{DirectoryOps, INode, Mode, NodeOps, NodeType, RegularOps, SymlinkOps},
        },
    },
};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use core::{
    any::Any,
    slice,
    sync::atomic::{AtomicU32, AtomicUsize, Ordering},
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
            mount_point: SpinMutex::default(),
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
            NodeType::SymbolicLink => NodeOps::SymbolicLink(Box::new(TmpRegular::default())),
            NodeType::Directory => NodeOps::Directory(Box::new(TmpDir::default())),
            _ => return Err(Errno::EINVAL),
        };
        let file_ops = Arc::try_new(TmpFile::default())?;

        Ok(Arc::try_new(INode {
            id: self.inode_counter.fetch_add(1, Ordering::Acquire) as u64,
            node_ops,
            file_ops,
            sb: self,
            cache: Arc::new(MemoryObject::new_phys()),
            mode: AtomicU32::new(mode.bits()),
            atime: SpinMutex::default(),
            mtime: SpinMutex::default(),
            ctime: SpinMutex::default(),
            size: AtomicUsize::default(),
            uid: AtomicUsize::default(),
            gid: AtomicUsize::default(),
        })?)
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
struct TmpDir;
impl DirectoryOps for TmpDir {
    fn lookup(&self, _: &Arc<INode>, _: &PathNode) -> EResult<()> {
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
            position: AtomicUsize::new(0),
        };
        return Ok(Arc::try_new(file)?);
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
struct TmpRegular {}

impl RegularOps for TmpRegular {
    fn truncate(&self, node: &INode, length: u64) -> EResult<()> {
        todo!()
    }
}

impl SymlinkOps for TmpRegular {
    fn read_link(&self, node: &INode, buf: &mut [u8]) -> EResult<u64> {
        let copy_size = buf.len().min(node.len());
        node.cache.read(&mut buf[0..copy_size], 0);
        Ok(copy_size as u64)
    }
}

#[derive(Debug, Default)]
struct TmpFile;
impl FileOps for TmpFile {
    fn read(&self, file: &File, buffer: &mut [u8], offset: uapi::off_t) -> EResult<isize> {
        let inode = file.inode.as_ref().ok_or(Errno::EINVAL)?;

        if offset as usize >= inode.len() {
            return Ok(0);
        }

        let copy_size = buffer.len().min(inode.len() - offset as usize);
        let actual = inode.cache.read(&mut buffer[0..copy_size], offset as usize);
        Ok(actual as _)
    }

    fn write(&self, file: &File, buffer: &[u8], offset: uapi::off_t) -> EResult<isize> {
        let inode = file.inode.as_ref().ok_or(Errno::EINVAL)?;
        let actual = inode.cache.write(buffer, offset as usize);
        inode.size.store(actual, Ordering::Release);
        Ok(actual as _)
    }

    fn seek(&self, file: &File, offset: SeekAnchor) -> EResult<uapi::off_t> {
        match offset {
            SeekAnchor::Start(x) => Ok(file.position.swap(x as usize, Ordering::AcqRel) as _),
            SeekAnchor::Current(x) => {
                let x = x as isize;
                let old = if x.is_negative() {
                    file.position.fetch_sub(x.unsigned_abs(), Ordering::AcqRel)
                } else {
                    file.position.fetch_add(x as _, Ordering::AcqRel)
                };
                Ok((old + x as usize) as _)
            }
            SeekAnchor::End(x) => {
                let x = x as isize;
                let size = file
                    .inode
                    .as_ref()
                    .ok_or(Errno::EINVAL)?
                    .size
                    .load(Ordering::Acquire);

                let new = if x.is_negative() {
                    size.checked_add_signed(x).ok_or(Errno::EINVAL)?
                } else {
                    size.checked_add_signed(x).ok_or(Errno::EOVERFLOW)?
                };

                file.position.store(new, Ordering::Release);
                Ok(new as _)
            }
        }
    }
}

#[initgraph::task(
    name = "generic.vfs.tmpfs",
    depends = [crate::generic::memory::MEMORY_STAGE],
    entails = [crate::generic::vfs::VFS_STAGE],
)]
pub fn TMPFS_INIT_STAGE() {
    super::register_fs(&TmpFs);
}
