// Virtual File System abstraction

use super::{handle::Handle, FileSystem};

pub struct VfsNode<'a> {
    handle: Option<&'a dyn Handle>,
    fs: &'a dyn FileSystem,
}

pub fn init() {
    // Create root node.

    // Mount temporary file systems.

    // Expose kernel logs to user space.

    todo!();
}
