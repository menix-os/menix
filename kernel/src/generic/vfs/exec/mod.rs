pub mod elf;

use crate::generic::{posix::errno::EResult, util::mutex::Mutex, vfs::file::File};
use alloc::{collections::btree_map::BTreeMap, string::String, sync::Arc};

/// Information passed to [`Executable::load`].
pub struct ExecutableInfo {
    /// The excutable to load.
    pub executable: Arc<File>,
    /// An interpreter that's tasked with loading the given executable.
    pub interpreter: Option<Arc<File>>,
}

pub trait Executable {
    /// Attempts to identify a file format.
    fn identify(&self, file: &File) -> bool;

    /// Loads an executable.
    fn parse(&self, info: &mut ExecutableInfo) -> EResult<()>;
}

static KNOWN_FORMATS: Mutex<BTreeMap<String, Arc<dyn Executable>>> = Mutex::new(BTreeMap::new());

/// Attempts to identify the format of this executable file.
pub fn identify(file: &File) -> Option<Arc<dyn Executable>> {
    let formats = KNOWN_FORMATS.lock();

    formats
        .iter()
        .find(|(_, f)| f.identify(file))
        .map(|(_, f)| f.clone())
}
