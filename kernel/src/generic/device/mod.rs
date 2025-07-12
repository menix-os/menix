use crate::generic::{
    posix::errno::{EResult, Errno},
    vfs::{file::FileOps, inode::Mode},
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::fmt::Debug;

pub trait Device {
    /// Returns the device ID.
    fn get_id(&self) -> uapi::dev_t;
    /// Returns the display name of this device.
    fn get_name(&self) -> &str;
    /// Returns the parent of this device.
    fn get_parent(&self) -> Option<Arc<dyn Device>>;
    /// Called when a new device is being connected.
    fn probe(&self) -> EResult<()>;
    /// Called when a device is being removed.
    fn remove(&self) -> EResult<()>;
    /// Called to put a device to sleep.
    fn suspend(&self) -> EResult<()>;
    /// Called to wake a device back up again.
    fn resume(&self) -> EResult<()>;
}

/// Represents a character device.
#[derive(Debug)]
pub struct CharDevice {
    pub id: uapi::dev_t,
    pub path: Vec<u8>,
    pub mode: Mode,
    pub ops: Arc<dyn FileOps>,
}

/// Represents a block device.
#[derive(Debug)]
pub struct BlockDevice {
    pub id: uapi::dev_t,
    pub path: Vec<u8>,
    pub mode: Mode,
    pub ops: Box<dyn BlockDeviceOps>,
}

// TODO: Document
pub trait BlockDeviceOps: Debug {
    fn get_block_size(&self) -> u64;

    fn get_block_count(&self) -> u64;

    fn write_data(&self, page_offset: u64, buffer: &[u8]) -> EResult<()>;

    fn poll(&self, mask: u16) -> EResult<u16>;

    fn ioctl(&self, request: usize, arg: usize) -> EResult<usize> {
        _ = (request, arg);
        Err(Errno::ENOTTY)
    }
}
