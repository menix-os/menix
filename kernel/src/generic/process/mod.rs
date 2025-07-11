pub mod task;

use crate::generic::{
    memory::{VirtAddr, virt::AddressSpace},
    percpu::CPU_DATA,
    posix::errno::{EResult, Errno},
    process::task::Task,
    util::{mutex::Mutex, once::Once},
    vfs::{self, cache::PathNode, exec::ExecInfo, file::File},
};
use alloc::{string::String, sync::Arc, vec::Vec};
use core::sync::atomic::{AtomicUsize, Ordering};

/// A unique process ID.
pub type Pid = usize;

pub struct Process {
    /// The unique identifier of this process.
    id: Pid,
    /// The display name of this process.
    name: String,
    /// The parent of this process.
    parent: Option<Arc<Process>>,
    /// Mutable fields of the process.
    pub inner: Mutex<InnerProcess>,
}

#[derive(Debug)]
pub struct InnerProcess {
    /// A list of associated tasks.
    threads: Vec<Arc<Task>>,
    /// The address space for this process.
    pub address_space: Arc<AddressSpace>,
    /// The root directory for this process.
    pub root_dir: PathNode,
    /// Current working directory.
    pub working_dir: PathNode,
    /// The user identity of this process.
    pub identity: Identity,
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

    pub fn get_parent(&self) -> Option<Arc<Self>> {
        self.parent.clone()
    }

    pub fn new(name: String, parent: Option<Arc<Self>>) -> EResult<Self> {
        let (root, cwd, identity) = match &parent {
            Some(x) => {
                let inner = x.inner.lock();
                (
                    inner.root_dir.clone(),
                    inner.working_dir.clone(),
                    inner.identity.clone(),
                )
            }
            None => (vfs::get_root(), vfs::get_root(), Identity::default()),
        };

        Ok(Self {
            id: PID_COUNTER.fetch_add(1, Ordering::Relaxed),
            name,
            parent,
            inner: Mutex::new(InnerProcess {
                threads: Vec::new(),
                address_space: Arc::new(AddressSpace::new()),
                root_dir: root,
                working_dir: cwd,
                identity,
            }),
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

    /// Replaces a process with a new executable image, given some arguments and an environment.
    /// The given file must be opened with ReadOnly and Executable.
    /// Any existing threads of the current process are destroyed upon a successful execve.
    /// This also means that a successful execve will never return.
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
        };

        let format = vfs::exec::identify(&file).ok_or(Errno::ENOEXEC)?;
        let init = Arc::new(format.load(&self, &mut info)?);

        // If we get here, then the loading of the executable was successful.
        let mut inner = self.inner.lock();
        inner.threads.clear();
        inner.threads.push(init.clone());
        inner.address_space = Arc::new(info.space);

        CPU_DATA.get().scheduler.add_task(init);
        CPU_DATA.get().scheduler.reschedule();

        Ok(())
    }
}

/// Entry point for tasks wanting to jump to user space.
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
    pub PROCESS_STAGE: "generic.process" => init;
}

fn init() {
    // Create the kernel process and task.
    unsafe {
        KERNEL_PROCESS.init(Arc::new(
            Process::new("kernel".into(), None).expect("Unable to create the main kernel process"),
        ))
    };
}
