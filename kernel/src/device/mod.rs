use crate::vfs::file::FileOps;

pub mod block;
mod console;
pub mod input;
mod memfiles;

/// Represents a generic character device.
pub trait CharDevice: FileOps {
    fn name(&self) -> &str;
}
