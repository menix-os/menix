//! File system abstractions.

use alloc::boxed::Box;

pub struct FileSystem {
    ops: Option<Box<dyn FileSystemOps>>,
}

pub trait FileSystemOps {}
