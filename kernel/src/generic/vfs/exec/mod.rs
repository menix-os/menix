pub mod elf;

use crate::generic::{posix::errno::EResult, util::mutex::Mutex, vfs::file::File};
use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    sync::Arc,
};

/// Information passed to [`ExecFormat::parse`].
pub struct ExecutableInfo {
    /// The excutable to load.
    pub executable: Arc<File>,
    /// An interpreter that's tasked with loading the given executable.
    pub interpreter: Option<Arc<File>>,
}

/// An executable format.
pub trait ExecFormat {
    /// Attempts to identify whether a file is a valid executable.
    fn identify(&self, file: &File) -> bool;

    /// Loads an executable.
    fn parse(&self, info: &mut ExecutableInfo) -> EResult<()>;
}

static KNOWN_FORMATS: Mutex<BTreeMap<String, Arc<dyn ExecFormat>>> = Mutex::new(BTreeMap::new());

/// Attempts to identify the format of this executable file.
pub fn identify(file: &File) -> Option<Arc<dyn ExecFormat>> {
    KNOWN_FORMATS
        .lock()
        .iter()
        .find(|(_, f)| f.identify(file))
        .map(|(_, f)| f.clone())
}

/// Installs a new executable format.
pub fn register(name: &str, format: Arc<dyn ExecFormat>) {
    KNOWN_FORMATS.lock().insert(name.to_string(), format);
}
