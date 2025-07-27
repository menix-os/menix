pub mod elf;

use crate::generic::{
    memory::virt::AddressSpace,
    posix::errno::EResult,
    process::{Process, task::Task},
    util::spin_mutex::SpinMutex,
    vfs::file::File,
};
use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    sync::Arc,
};

/// Information passed to [`ExecFormat::load`].
#[derive(Debug)]
pub struct ExecInfo<'a> {
    /// The excutable to load.
    pub executable: Arc<File>,
    /// An interpreter that's tasked with loading the given executable.
    pub interpreter: Option<Arc<File>>,
    /// An address space for the new process.
    pub space: AddressSpace,
    /// Arguments.
    pub argv: &'a [&'a [u8]],
    /// Environment variables.
    pub envp: &'a [&'a [u8]],
}

/// An executable format.
pub trait ExecFormat {
    /// Identifies whether a file is a valid executable of this format.
    fn identify(&self, file: &File) -> bool;

    /// Loads the executable and returns a new initial thread.
    /// The implementation should not modify `old` at all.
    fn load(&self, proc: &Arc<Process>, info: &mut ExecInfo) -> EResult<Task>;
}

static KNOWN_FORMATS: SpinMutex<BTreeMap<String, Arc<dyn ExecFormat>>> =
    SpinMutex::new(BTreeMap::new());

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
