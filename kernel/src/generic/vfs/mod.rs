pub mod cache;
pub mod exec;
pub mod file;
pub mod fs;
pub mod inode;

use crate::generic::{
    posix::errno::{EResult, Errno},
    process::Identity,
    util::once::Once,
    vfs::{
        cache::LookupFlags,
        inode::{Mode, NodeType},
    },
};
use alloc::sync::Arc;

pub use cache::Entry;
pub use cache::PathNode;
pub use file::File;
pub use fs::Mount;
pub use fs::MountFlags;

/// The root directory entry.
static ROOT: Once<PathNode> = Once::new();

/// Gets a reference to the root of the VFS.
pub fn get_root() -> PathNode {
    ROOT.get().clone()
}

/// Creates a new node in the VFS.
pub fn mknod(
    at: Option<Arc<File>>,
    path: &[u8],
    file_type: NodeType,
    mode: Mode,
    device: Option<()>, // TODO
    identity: &Identity,
) -> EResult<()> {
    // POSIX only allows these types of nodes to be created.
    match file_type {
        NodeType::Regular
        | NodeType::Directory
        | NodeType::BlockDevice
        | NodeType::CharacterDevice
        | NodeType::FIFO => (),
        _ => return Err(Errno::EINVAL),
    }

    let path = PathNode::flookup(at, path, identity, LookupFlags::MustNotExist)?;
    let parent_inode = path
        .entry
        .parent
        .as_ref()
        .and_then(|p| p.get_inode())
        .expect("Entry has no parent node?");

    let new_inode = parent_inode.sb.clone().create_inode(file_type, mode)?;
    path.entry.set_inode(new_inode);

    Ok(())
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
