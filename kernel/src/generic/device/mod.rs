mod console;
mod memfiles;

use crate::generic::{posix::errno::EResult, vfs::file::FileOps};
use core::fmt::Debug;

/// Represents a generic device.
pub trait Device: Debug + FileOps {
    fn name(&self) -> &str;
    fn open(&self) -> EResult<()>;
}
