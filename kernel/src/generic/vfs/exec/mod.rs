pub mod elf;

use crate::generic::{
    memory::virt::AddressSpace,
    posix::errno::EResult,
    process::{Process, task::Task},
    util::mutex::Mutex,
    vfs::file::File,
};
use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};

/// Information passed to [`ExecFormat::load`].
#[derive(Debug)]
pub struct ExecInfo {
    /// The excutable to load.
    pub executable: Arc<File>,
    /// An interpreter that's tasked with loading the given executable.
    pub interpreter: Option<Arc<File>>,
    /// An address space for the new process.
    pub space: AddressSpace,
    /// How many arguments are living on the stack.
    pub argc: usize,
    /// How many environment variables are living on the stack.
    pub envc: usize,
    /// A list of tasks to create.
    pub tasks: Vec<Task>,
}

/// An executable format.
pub trait ExecFormat {
    /// Identifies whether a file is a valid executable of this format.
    fn identify(&self, file: &File) -> bool;

    /// Loads the executable and returns a new process.
    fn load(&self, old: &Arc<Process>, info: &mut ExecInfo) -> EResult<()>;
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
