pub mod cache;
pub mod exec;
pub mod file;
pub mod fs;
pub mod inode;

use crate::generic::{
    util::once::Once,
    vfs::cache::{MountFlags, PathNode},
};

/// The root directory entry.
static ROOT: Once<PathNode> = Once::new();

/// Gets a reference to the root of the VFS.
pub fn get_root() -> PathNode {
    ROOT.get().clone()
}

init_stage! {
    #[depends(crate::generic::memory::MEMORY_STAGE)]
    pub VFS_STAGE: "generic.vfs" => init;
}

fn init() {
    // Mount a tmpfs as root.
    let initrd_mount =
        fs::mount(None, b"tmpfs", MountFlags::empty()).expect("Unable to mount the tmpfs");

    let root_path = PathNode {
        entry: initrd_mount.root.clone(),
        mount: initrd_mount,
    };

    unsafe { ROOT.init(root_path) };
}
