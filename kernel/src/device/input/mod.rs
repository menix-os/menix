use super::CharDevice;

/// Represents a Linux evdev-compatible input device.
pub trait EventDevice: CharDevice {}
