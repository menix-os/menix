pub mod entry;
pub mod exec;
pub mod file;
pub mod fs;
pub mod inode;
pub mod path;

use crate::generic::util::mutex::Mutex;
use alloc::sync::Arc;
use entry::Entry;

/// The root of the VFS.
static ROOT: Mutex<Option<Arc<Entry>>> = Mutex::new(None);

pub(crate) fn init() {
    // fs::register_fs(Box::new(fs::tmpfs::TmpFs));

    // Mount the tmpfs as root.
    // TODO
}
