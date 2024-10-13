use core::error::Error;

pub mod fd;
pub mod handle;
pub mod vfs;

/// Describes a file system.
pub trait FileSystem {
    /// Returns the name of the file system.
    fn name(&self) -> &'static str;
}
