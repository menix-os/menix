pub mod elf;

use crate::generic::{posix::errno::EResult, resource::Resource};
use alloc::boxed::Box;

/// Information passed to [`Executable::load`].
pub struct ExecutableInfo {
    /// The excutable to load.
    pub executable: Box<dyn Resource>,
    /// An interpreter that's tasked with loading the given executable.
    pub interpreter: Option<Box<dyn Resource>>,
}

pub trait Executable {
    /// Loads an executable.
    fn load(&self, info: &mut ExecutableInfo) -> EResult<()>;
}
