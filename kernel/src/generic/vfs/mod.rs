pub mod cache;
pub mod exec;
pub mod file;
pub mod fs;
pub mod inode;

use crate::generic::{
    posix::errno::EResult,
    util::once::Once,
    vfs::inode::{Mode, NodeType},
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
) -> EResult<()> {
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
