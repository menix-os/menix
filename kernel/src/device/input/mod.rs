use crate::{
    posix::errno::EResult,
    vfs::{File, file::FileOps},
};

use super::CharDevice;

/// Represents a Linux evdev-compatible input device.
pub struct EventDevice {}

impl CharDevice for EventDevice {
    fn name(&self) -> &str {
        "evdev"
    }
}

impl FileOps for EventDevice {
    fn read(&self, file: &File, buffer: &mut [u8], offset: u64) -> EResult<isize> {
        let _ = (file, buffer, offset);
        todo!()
    }
}

pub struct EventListener {}

impl EventListener {}
