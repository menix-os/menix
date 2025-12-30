use crate::{
    posix::errno::EResult,
    uapi,
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

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InputEvent {
    pub time: uapi::time::timeval,
    pub typ: u16,
    pub code: u16,
    pub value: i32,
}
static_assert!(size_of::<InputEvent>() == 24);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InputId {
    pub bustype: u16,
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InputAbsinfo {
    pub value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub fuzz: i32,
    pub flat: i32,
    pub resolution: i32,
}
