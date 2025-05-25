use crate::generic::{boot::BootInfo, util::once::Once};
use alloc::sync::Arc;
use inode::INode;

pub mod entry;
pub mod file;
pub mod fs;
pub mod inode;
pub mod path;

/// The root of the VFS.
static ROOT: Once<Arc<INode>> = Once::new();

pub(crate) fn init() {
    for file in BootInfo::get().files {
        fs::initrd::load(file.data, ROOT.get().clone());
    }
}
