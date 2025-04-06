use super::{errno::Errno, memory::virt::ForeignPtr};
use alloc::boxed::Box;

/// Represents a single node in the virtual file system.
pub struct Node {
    ops: Option<Box<dyn NodeOps>>,
}

pub trait NodeOps {
    /// Reads from the node, starting at `offset`, into a buffer. Returns actual bytes read.
    fn read(&self, buffer: &mut [u8], offset: usize) -> Result<usize, Errno>;
    fn write(&mut self, buffer: &[u8], offset: usize) -> usize;
    fn ioctl(&mut self, request: usize, arg: ForeignPtr<u8>);
    fn access(&mut self);
}

pub struct FileSystem {
    ops: Option<Box<dyn FileSystemOps>>,
}

pub trait FileSystemOps {}
