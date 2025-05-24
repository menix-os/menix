//! File system abstractions.

pub mod dir;
pub mod file;
pub mod node;
pub mod path;

mod ramdisk;
mod tmpfs;

use core::{any::Any, fmt::Debug};

use crate::generic::{boot::BootInfo, util::once::Once};
use alloc::sync::Arc;
use node::Node;

#[derive(Debug)]
pub struct FileSystem {
    name: &'static str,
    ops: &'static dyn FileSystemOps,
}

impl FileSystem {
    pub fn new(name: &'static str, ops: &'static dyn FileSystemOps) -> Self {
        Self { name, ops }
    }
}

pub trait FileSystemOps: Any + Debug {}

static ROOT_NODE: Once<Arc<Node>> = Once::new();

pub fn init() {
    unsafe { ROOT_NODE.init(Node::new().unwrap()) };

    for file in BootInfo::get().files {
        ramdisk::load(file.data, ROOT_NODE.get().clone());
    }
}
