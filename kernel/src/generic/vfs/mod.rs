pub mod cache;
pub mod exec;
pub mod file;
pub mod fs;
pub mod inode;

pub use cache::Entry;
pub use cache::PathNode;
pub use file::File;
pub use fs::Mount;
pub use fs::MountFlags;

use crate::generic::memory::cache::MemoryObject;
use crate::generic::process::InnerProcess;
use crate::generic::{
    device::Device,
    memory::{
        VirtAddr,
        virt::{AddressSpace, VmFlags},
    },
    posix::errno::{EResult, Errno},
    process::Identity,
    util::once::Once,
    vfs::{
        cache::LookupFlags,
        file::{MmapFlags, OpenFlags},
        inode::{Mode, NodeOps, NodeType},
    },
};
use alloc::sync::Arc;
use core::num::NonZeroUsize;

/// The root directory entry.
static ROOT: Once<PathNode> = Once::new();

/// Gets a reference to the root of the VFS.
pub fn get_root() -> PathNode {
    ROOT.get().clone()
}

/// Creates a new node in the VFS.
pub fn mknod(
    inner: &InnerProcess,
    at: Option<Arc<File>>,
    path: &[u8],
    file_type: NodeType,
    mode: Mode,
    device: Option<Arc<dyn Device>>,
    identity: &Identity,
) -> EResult<()> {
    match file_type {
        // POSIX only allows these types of nodes to be created.
        NodeType::Regular
        | NodeType::Directory
        | NodeType::BlockDevice
        | NodeType::CharacterDevice
        | NodeType::FIFO => (),
        _ => return Err(Errno::EINVAL),
    }

    let path = PathNode::flookup(inner, at, path, identity, LookupFlags::MustNotExist)?;
    let parent = path
        .lookup_parent()
        .and_then(|p| p.entry.get_inode().ok_or(Errno::ENOENT))
        .expect("Entry has no parent node?");

    let new_inode = parent.sb.clone().create_inode(file_type, mode)?;
    path.entry.set_inode(new_inode);

    Ok(())
}

/// Creates a symbolic link at `path`, pointing to `target_path`.
pub fn symlink(
    inner: &InnerProcess,
    at: Option<Arc<File>>,
    path: &[u8],
    target_path: &[u8],
    identity: &Identity,
) -> EResult<()> {
    let path = PathNode::lookup(
        inner,
        at.and_then(|x| x.path.clone()),
        path,
        identity,
        LookupFlags::MustNotExist,
    )?;

    let parent_inode = path
        .lookup_parent()?
        .entry
        .get_inode()
        .ok_or(Errno::ENOENT)?;
    parent_inode.try_access(identity, OpenFlags::WriteOnly, false)?;

    // Create the symlink in the parent directory.
    match &parent_inode.node_ops {
        NodeOps::Directory(x) => x.symlink(&parent_inode, path, target_path, identity),
        _ => return Err(Errno::ENOTDIR),
    }
}

/// Maps a file in the address space of a process.
pub fn mmap(
    file: Option<Arc<File>>,
    space: &AddressSpace,
    addr: VirtAddr,
    len: NonZeroUsize,
    prot: VmFlags,
    flags: MmapFlags,
    offset: uapi::off_t,
) -> EResult<VirtAddr> {
    if flags.contains(MmapFlags::Anonymous) {
        let anon = Arc::new(MemoryObject::new_phys());
        space.map_object(anon, addr, len, prot, offset)?;
    } else if let Some(f) = file {
        let object = f.get_memory_object(len, offset, flags.contains(MmapFlags::Private))?;
        space.map_object(object, addr, len, prot, offset)?;
    } else {
        return Err(Errno::EINVAL);
    }

    return Ok(addr);
}

init_stage! {
    #[depends(crate::generic::memory::MEMORY_STAGE)]
    pub VFS_STAGE: "generic.vfs" => init;
}

fn init() {
    // Mount a tmpfs as root.
    let tmpfs =
        fs::mount(None, b"tmpfs", MountFlags::empty()).expect("Unable to mount the root tmpfs");

    let root_path = PathNode {
        entry: tmpfs.root.clone(),
        mount: tmpfs,
    };

    unsafe { ROOT.init(root_path.clone()) };
}
