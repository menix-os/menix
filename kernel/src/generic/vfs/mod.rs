pub mod entry;
pub mod exec;
pub mod file;
pub mod fs;
pub mod inode;
pub mod path;

use crate::generic::{
    posix::errno::{EResult, Errno},
    util::mutex::Mutex,
};
use alloc::sync::Arc;
use entry::Entry;

static ROOT: Mutex<Option<Arc<Entry>>> = Mutex::new(None);

/// Gets a reference to the root of the VFS.
/// May return [`Errno::ENOENT`] if there is no root entry.
pub fn get_root() -> EResult<Arc<Entry>> {
    match &*ROOT.lock() {
        Some(x) => Ok(x.clone()),
        None => Err(Errno::ENOENT),
    }
}

pub(crate) fn init() {
    // fs::register_fs(Box::new(fs::tmpfs::TmpFs));

    // Mount the tmpfs as root.
    // TODO
}
