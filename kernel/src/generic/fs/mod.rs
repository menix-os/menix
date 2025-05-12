//! File system abstractions.

pub mod file;

use alloc::boxed::Box;

pub struct FileSystem {
    ops: Option<Box<dyn FileSystemOps>>,
}

pub trait FileSystemOps {}
