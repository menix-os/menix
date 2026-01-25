#![allow(unused)]

use super::{MountFlags, SuperBlock};
use crate::{
    arch,
    memory::{AddressSpace, PagedMemoryObject, PhysAddr, VirtAddr, VmFlags, cache::MemoryObject},
    posix::errno::{EResult, Errno},
    process::Identity,
    uapi::{self, statvfs::statvfs},
    util::mutex::{Mutex, spin::SpinMutex},
    vfs::{
        PathNode,
        cache::Entry,
        file::{File, FileOps, MmapFlags, OpenFlags, SeekAnchor},
        fs::{FileSystem, Mount},
        inode::{DirectoryOps, INode, Mode, NodeOps, NodeType, RegularOps, SymlinkOps},
    },
};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use core::{
    any::Any,
    num::NonZeroUsize,
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

        let dir = Arc::new(TmpDir);
        let root_inode = super_block.clone().create_inode(
            NodeOps::Directory(dir.clone()),
            dir,
            Mode::from_bits_truncate(0o755),
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

    fn statvfs(self: Arc<Self>) -> EResult<statvfs> {
        todo!()
    }

    fn create_inode(
        self: Arc<Self>,
        node_ops: NodeOps,
        file_ops: Arc<dyn FileOps>,
        mode: Mode,
    ) -> EResult<Arc<INode>> {
        Ok(Arc::try_new(INode {
            id: self.inode_counter.fetch_add(1, Ordering::Acquire),
            node_ops,
            file_ops,
            sb: self,
            mode: SpinMutex::new(mode),
            atime: SpinMutex::default(),
            mtime: SpinMutex::default(),
            ctime: SpinMutex::default(),
            size: SpinMutex::default(),
            uid: SpinMutex::default(),
            gid: SpinMutex::default(),
        })?)
    }

    fn destroy_inode(self: Arc<Self>, inode: INode) -> EResult<()> {
        match Arc::into_inner(self) {
            Some(x) => Ok(()),
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
            offset: Mutex::new(0),
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

    fn symlink(
        &self,
        node: &Arc<INode>,
        path: PathNode,
        target_path: &[u8],
        identity: &Identity,
    ) -> EResult<()> {
        let _ = identity; // TODO
        let reg = Arc::new(TmpSymlink::default());
        let node_ops = NodeOps::SymbolicLink(reg.clone());

        let sym_inode =
            node.sb
                .clone()
                .create_inode(node_ops, reg.clone(), Mode::from_bits_truncate(0o777))?;

        *reg.target.lock() = target_path.to_vec();
        *sym_inode.size.lock() = target_path.len();
        path.entry.set_inode(sym_inode);
        Ok(())
    }

    fn create(&self, self_node: &Arc<INode>, entry: Arc<Entry>, mode: Mode) -> EResult<()> {
        let mut children = entry.children.lock();
        let new_file = Arc::new(TmpRegular::new());
        let new_node = self_node.sb.clone().create_inode(
            NodeOps::Regular(new_file.clone()),
            new_file,
            mode,
        )?;
        entry.set_inode(new_node);
        Ok(())
    }

    fn mkdir(&self, self_node: &Arc<INode>, entry: Arc<Entry>, mode: Mode) -> EResult<Arc<Entry>> {
        let mut children = entry.children.lock();
        let new_dir = Arc::new(TmpDir);
        let new_dir_node = self_node.sb.clone().create_inode(
            NodeOps::Directory(new_dir.clone()),
            new_dir,
            mode,
        )?;
        entry.set_inode(new_dir_node);
        Ok(entry.clone()) // TODO: This is wrong. Return the child entry instead.
    }

    fn mknod(
        &self,
        self_node: &Arc<INode>,
        node_type: NodeType,
        mode: Mode,
        dev: Option<Arc<dyn FileOps>>,
    ) -> EResult<Arc<INode>> {
        let new_node = dev.ok_or(Errno::ENODEV)?;
        self_node.sb.clone().create_inode(
            match node_type {
                NodeType::BlockDevice => NodeOps::BlockDevice,
                NodeType::CharacterDevice => NodeOps::CharacterDevice,
                _ => return Err(Errno::ENODEV),
            },
            new_node,
            mode,
        )
    }
}

#[derive(Debug, Default)]
struct TmpSymlink {
    pub target: SpinMutex<Vec<u8>>,
}

impl SymlinkOps for TmpSymlink {
    fn read_link(&self, node: &INode, buf: &mut [u8]) -> EResult<u64> {
        let target = self.target.lock();
        let copy_size = buf.len().min(target.len());
        buf[0..copy_size].copy_from_slice(&target[0..copy_size]);
        Ok(copy_size as u64)
    }
}

impl FileOps for TmpSymlink {}
impl FileOps for TmpDir {}

#[derive(Debug)]
struct TmpRegular {
    /// A mappable page cache for the contents of the node.
    pub cache: Arc<PagedMemoryObject>,
}

impl TmpRegular {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(PagedMemoryObject::new_phys()),
        }
    }
}

impl RegularOps for TmpRegular {
    fn truncate(&self, node: &INode, length: u64) -> EResult<()> {
        todo!()
    }
}

impl FileOps for TmpRegular {
    fn read(&self, file: &File, buffer: &mut [u8], offset: u64) -> EResult<isize> {
        let inode = file.inode.as_ref().ok_or(Errno::EINVAL)?;
        let start = offset;

        if start as usize >= inode.len() {
            return Ok(0);
        }

        let copy_size = buffer.len().min(inode.len() - start as usize);
        let actual = (self.cache.as_ref() as &dyn MemoryObject)
            .read(&mut buffer[0..copy_size], start as usize);

        Ok(actual as _)
    }

    fn write(&self, file: &File, buffer: &[u8], offset: u64) -> EResult<isize> {
        let inode = file.inode.as_ref().ok_or(Errno::EINVAL)?;
        let mut size_lock = inode.size.lock();
        let start = offset;
        let actual = (self.cache.as_ref() as &dyn MemoryObject).write(buffer, start as usize);
        *size_lock = actual;

        Ok(actual as _)
    }

    fn mmap(
        &self,
        file: &File,
        space: &mut AddressSpace,
        addr: VirtAddr,
        len: NonZeroUsize,
        prot: VmFlags,
        flags: MmapFlags,
        offset: uapi::off_t,
    ) -> EResult<VirtAddr> {
        let object = if flags.contains(MmapFlags::Private) {
            self.cache.make_private(len, offset)?
        } else {
            self.cache.clone()
        };

        let page_size = arch::virt::get_page_size();
        let misalign = addr.value() & (page_size - 1);
        let map_address = addr - misalign;
        let backed_map_size = (len.get() + misalign + page_size - 1) & !(page_size - 1);

        space.map_object(
            object,
            map_address,
            NonZeroUsize::new(backed_map_size).unwrap(),
            prot,
            offset - misalign as isize,
        )?;
        Ok(addr)
    }
}

#[initgraph::task(
    name = "generic.vfs.tmpfs",
    depends = [crate::memory::MEMORY_STAGE],
    entails = [crate::vfs::VFS_STAGE],
)]
pub fn TMPFS_INIT_STAGE() {
    super::register_fs(&TmpFs);
}
