use crate::generic::{posix::errno::EResult, vfs::File};
use alloc::sync::Arc;
use core::fmt::Debug;

/// Represents a generic device.
pub trait Device: Debug {
    fn open(&self) -> EResult<Arc<File>>;
}

pub fn register(device: Arc<dyn Device>) -> EResult<()> {
    todo!()
}
