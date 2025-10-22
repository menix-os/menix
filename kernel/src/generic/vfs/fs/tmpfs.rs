#![allow(unused)]

use super::{MountFlags, SuperBlock};
use crate::{
    arch,
    generic::{
        device::Device,
        memory::{PhysAddr, cache::MemoryObject},
        posix::errno::{EResult, Errno},
        process::Identity,
        util::mutex::{Mutex, spin::SpinMutex},
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
    sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering},
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

        let root_inode = super_block.clone().create_inode(
            NodeType::Directory,
            Mode::from_bits_truncate(0o755),
            None,
        )?;

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

    fn create_inode(
        self: Arc<Self>,
        node_type: NodeType,
        mode: Mode,
        device: Option<Arc<dyn Device>>,
    ) -> EResult<Arc<INode>> {
        let (node_ops, file_ops): (_, Arc<dyn FileOps>) = match node_type {
            NodeType::Regular => {
                let reg = Arc::new(TmpRegular::default());
                (NodeOps::Regular(reg.clone()), reg)
            }
            NodeType::SymbolicLink => {
                let reg = Arc::new(TmpSymlink::default());
                (NodeOps::SymbolicLink(reg.clone()), reg)
            }
            NodeType::Directory => {
                let reg = Arc::new(TmpDir::default());
                (NodeOps::Directory(reg.clone()), reg)
            }
            NodeType::CharacterDevice => {
                let dev = device.ok_or(Errno::ENODEV)?;
                (NodeOps::CharacterDevice(dev.clone()), dev)
            }
            NodeType::BlockDevice => {
                let dev = device.ok_or(Errno::ENODEV)?;
                (NodeOps::BlockDevice(dev.clone()), dev)
            }
            _ => return Err(Errno::EINVAL),
        };

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
            ops: node.file_ops.clone(),
            inode: Some(node.clone()),
            flags: Mutex::new(flags),
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
struct TmpSymlink {}

impl SymlinkOps for TmpSymlink {
    fn read_link(&self, node: &INode, buf: &mut [u8]) -> EResult<u64> {
        let copy_size = buf.len().min(node.len());
        node.cache.read(&mut buf[0..copy_size], 0);
        Ok(copy_size as u64)
    }
}

impl FileOps for TmpSymlink {}
impl FileOps for TmpDir {}

#[derive(Debug, Default)]
struct TmpRegular {
    position: AtomicU64,
}

impl RegularOps for TmpRegular {
    fn truncate(&self, node: &INode, length: u64) -> EResult<()> {
        todo!()
    }
}

impl FileOps for TmpRegular {
    fn release(&self, file: &File) -> EResult<()> {
        self.position.store(0, Ordering::Release);
        Ok(())
    }

    fn read(&self, file: &File, buffer: &mut [u8], offset: Option<u64>) -> EResult<isize> {
        let inode = file.inode.as_ref().ok_or(Errno::EINVAL)?;
        let start = offset.unwrap_or(self.position.load(Ordering::Acquire) as _);

        if start as usize >= inode.len() {
            return Ok(0);
        }

        let copy_size = buffer.len().min(inode.len() - start as usize);
        let actual = inode.cache.read(&mut buffer[0..copy_size], start as usize) as _;

        // If there was no offset specified, advance the internal cursor.
        if offset.is_none() {
            self.position.fetch_add(actual, Ordering::AcqRel);
        }

        Ok(actual as _)
    }

    fn write(&self, file: &File, buffer: &[u8], offset: Option<u64>) -> EResult<isize> {
        let inode = file.inode.as_ref().ok_or(Errno::EINVAL)?;
        let start = offset.unwrap_or(self.position.load(Ordering::Acquire));
        let actual = inode.cache.write(buffer, start as usize);
        inode.size.store(actual, Ordering::Release);

        // If there was no offset specified, advance the internal cursor.
        if offset.is_none() {
            self.position.fetch_add(actual as _, Ordering::AcqRel);
        }

        Ok(actual as _)
    }

    fn seek(&self, file: &File, offset: SeekAnchor) -> EResult<u64> {
        let new = match offset {
            SeekAnchor::Start(x) => {
                self.position.store(x, Ordering::Release);
                Ok(x)
            }
            SeekAnchor::Current(x) => {
                let x = x as i64;
                let old = if x.is_negative() {
                    self.position.fetch_sub(x.unsigned_abs(), Ordering::AcqRel)
                } else {
                    self.position.fetch_add(x as _, Ordering::AcqRel)
                };
                old.checked_add_signed(x).ok_or(Errno::EOVERFLOW)
            }
            SeekAnchor::End(x) => {
                let size = file
                    .inode
                    .as_ref()
                    .ok_or(Errno::EINVAL)?
                    .size
                    .load(Ordering::Acquire);

                let new = if x.is_negative() {
                    size.checked_add_signed(x as _).ok_or(Errno::EINVAL)?
                } else {
                    size.checked_add_signed(x as _).ok_or(Errno::EOVERFLOW)?
                };

                self.position.store(new as _, Ordering::Release);
                Ok(new as _)
            }
        };
        new
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
