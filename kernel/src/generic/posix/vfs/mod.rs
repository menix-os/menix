use crate::generic::{boot::BootInfo, util::once::Once};
use alloc::{sync::Arc, vec::Vec};
use entry::Entry;

pub mod entry;
pub mod exec;
pub mod file;
pub mod fs;
pub mod inode;
pub mod path;

/// The root of the VFS.
static ROOT: Once<Arc<Entry>> = Once::new();

pub(crate) fn init() {
    unsafe { ROOT.init(entry::Entry::new(Vec::new(), None, None)) };

    // Mount all initial ramdisks on the root.
    for file in BootInfo::get().files {
        fs::initrd::load(file.data, ROOT.get().clone());
    }
}
