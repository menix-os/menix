// File descriptor

use super::{handle::Handle, vfs::VfsNode};

pub struct FileDescriptor<'a> {
    /// Handle connected to this descriptor.
    handle: &'a dyn Handle,
    /// Current offset into the file.
    offset: usize,
    /// The node that this descriptor is pointing to.
    node: &'a VfsNode<'a>,
}
