mod memfiles;

use crate::generic::{posix::errno::EResult, vfs::File};
use alloc::{string::String, sync::Arc};
use core::fmt::Debug;

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub ops: &'static dyn DeviceOps,
}

impl Device {
    pub fn open(&self) -> EResult<Arc<File>> {
        self.ops.open(self)
    }
}

/// Represents a generic device.
pub trait DeviceOps: Debug {
    fn open(&self, dev: &Device) -> EResult<Arc<File>>;
}
