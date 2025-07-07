pub mod sched;
pub mod task;

use crate::generic::{
    memory::{VirtAddr, virt::AddressSpace},
    posix::errno::{EResult, Errno},
    process::task::Tid,
    util::{mutex::Mutex, once::Once},
    vfs::{self, cache::PathNode, exec::ExecInfo, file::File},
};
use alloc::{
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use core::sync::atomic::{AtomicUsize, Ordering};

/// A unique process ID.
pub type Pid = usize;

/// Represents a process and address space.
#[derive(Debug)]
pub struct Process {
    /// The unique identifier of this process.
    id: Pid,
    /// The display name of this process.
    name: String,
    /// If this process is a user process or not.
    is_user: bool,
    /// A list of associated tasks.
    threads: Mutex<Vec<Tid>>,
    /// The address space for this process.
    pub address_space: Arc<AddressSpace>,
    /// The root directory for this process.
    pub root_dir: Mutex<PathNode>,
    /// Current working directory.
    pub working_dir: Mutex<PathNode>,
    /// The user identity of this process.
    pub identity: Mutex<Identity>,
    /// The parent of this process.
    pub parent: Option<Arc<Self>>,
}

impl Process {
    /// Returns the unique identifier of this process.
    #[inline]
    pub const fn get_pid(&self) -> Pid {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn new(name: &str, parent: Option<Arc<Self>>, is_user: bool) -> EResult<Self> {
        let root = match &parent {
            Some(x) => x.root_dir.lock().clone(),
            None => vfs::get_root(),
        };

        let cwd = match &parent {
            Some(x) => x.working_dir.lock().clone(),
            None => vfs::get_root(),
        };

        let identity = match &parent {
            Some(x) => x.identity.lock().clone(),
            None => Identity::default(),
        };

        Ok(Self {
            id: PID_COUNTER.fetch_add(1, Ordering::Relaxed),
            name: name.to_string(),
            address_space: Arc::try_new(AddressSpace::new())?,
            threads: Mutex::new(Vec::new()),
            root_dir: Mutex::new(root),
            working_dir: Mutex::new(cwd),
            is_user,
            identity: Mutex::new(identity),
            parent,
        })
    }

    /// Returns the kernel process.
    pub fn get_kernel() -> Arc<Self> {
        KERNEL_PROCESS.get().clone()
    }

    /// Forks a process into a new one.
    pub fn fork(self: Arc<Self>) -> Self {
        todo!()
    }

    /// Replaces a process with a new executable image.
    /// The given file must be opened with [`OpenFlags::ReadOnly`] and [`OpenFlags::Executable`].
    /// Any existing threads of the current process are destroyed upon a successful execve.
    pub fn fexecve(
        self: Arc<Self>,
        file: Arc<File>,
        argv: &[&[u8]],
        envp: &[&[u8]],
    ) -> EResult<()> {
        let mut info = ExecInfo {
            executable: file.clone(),
            interpreter: None,
            space: AddressSpace::new(),
            argc: argv.len(),
            envc: envp.len(),
            tasks: Vec::new(),
        };

        let format = vfs::exec::identify(&file).ok_or(Errno::ENOEXEC)?;
        format.load(&self, &mut info)?;

        // If we get here, then the loading of the executable was successful.

        // Replace the old address space.
        self.address_space.clear();

        Ok(())
    }
}

pub extern "C" fn to_user(ip: usize, sp: usize) {
    unsafe { crate::arch::sched::jump_to_user(VirtAddr::from(ip), VirtAddr::from(sp)) };
}

#[derive(Debug, Clone, Default)]
pub struct Identity {
    pub user_id: uapi::uid_t,
    pub group_id: uapi::gid_t,

    pub effective_user_id: uapi::uid_t,
    pub effective_group_id: uapi::gid_t,

    pub set_user_id: uapi::uid_t,
    pub set_group_id: uapi::gid_t,

    pub groups: Vec<uapi::gid_t>,
}

impl Identity {
    /// Returns an identity suitable for kernel accesses, with absolute privileges for everything.
    pub fn get_kernel() -> &'static Identity {
        static KERNEL_IDENTITY: Identity = Identity {
            user_id: 0,
            group_id: 0,
            effective_user_id: 0,
            effective_group_id: 0,
            set_user_id: 0,
            set_group_id: 0,
            groups: vec![],
        };
        &KERNEL_IDENTITY
    }
}

static PID_COUNTER: AtomicUsize = AtomicUsize::new(0);
static KERNEL_PROCESS: Once<Arc<Process>> = Once::new();

init_stage! {
    #[depends(crate::generic::memory::MEMORY_STAGE, crate::generic::vfs::VFS_STAGE)]
    PROCESS_STAGE: "generic.process" => init;
}

fn init() {
    // Create the kernel process and task.
    unsafe {
        KERNEL_PROCESS.init(Arc::new(
            Process::new("kernel", None, false).expect("Unable to create the main kernel process"),
        ))
    };
}
