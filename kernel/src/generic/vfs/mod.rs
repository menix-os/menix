pub mod entry;
pub mod exec;
pub mod file;
pub mod fs;
pub mod inode;
pub mod path;

use crate::generic::{posix::errno::EResult, util::mutex::Mutex};
use alloc::{boxed::Box, sync::Arc};
use entry::Entry;
use {
    file::{File, OpenFlags},
    path::PathBuf,
};

/// The root of the VFS.
static ROOT: Mutex<Option<Arc<Entry>>> = Mutex::new(None);

pub(crate) fn init() {
    // fs::register_fs(Box::new(fs::tmpfs::TmpFs));

    // Mount the tmpfs as root.
    // TODO
}
