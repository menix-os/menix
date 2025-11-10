pub mod task;

use crate::{
    arch::sched::Context,
    {
        memory::{VirtAddr, virt::AddressSpace},
        percpu::CpuData,
        posix::errno::{EResult, Errno},
        process::task::Task,
        sched::Scheduler,
        util::{mutex::spin::SpinMutex, once::Once},
        vfs::{
            self,
            cache::PathNode,
            exec::ExecInfo,
            file::{File, FileDescription},
        },
    },
};
use alloc::{
    boxed::Box,
    collections::{btree_map::BTreeMap, btree_set::BTreeSet},
    string::String,
    sync::{Arc, Weak},
    vec::Vec,
};
use core::sync::atomic::{AtomicUsize, Ordering};

/// A unique process ID.
pub type Pid = usize;

#[derive(Debug)]
pub enum ProcessState {
    Running,
    Exited(u8),
    // TODO: SIGSTOP
}

#[derive(Debug)]
pub struct Process {
    /// The unique identifier of this process.
    id: Pid,
    /// The display name of this process.
    name: String,
    /// The parent of this process, or [`None`], if this is the init process.
    parent: Option<Weak<Process>>,
    /// Mutable fields of the process.
    pub inner: SpinMutex<InnerProcess>,
}

/// The lockable, mutable part of a process.
#[derive(Debug)]
pub struct InnerProcess {
    pub status: ProcessState,
    /// A list of associated tasks.
    pub threads: Vec<Arc<Task>>,
    /// Child processes of this process.
    pub children: Vec<Arc<Process>>,
    /// The address space for this process.
    pub address_space: Arc<AddressSpace>,
    /// The root directory for this process.
    pub root_dir: PathNode,
    /// Current working directory.
    pub working_dir: PathNode,
    /// The user identity of this process.
    pub identity: Identity,
    /// A table of open file descriptors.
    pub open_files: BTreeMap<usize, FileDescription>,
    /// A pointer to the next free memory region.
    pub mmap_head: VirtAddr,
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

    /// Gets the parent process of this process.
    /// Returns [`None`], if it is the init process.
    pub fn get_parent(&self) -> Option<Arc<Self>> {
        // TODO: The upgrade should never fail.
        // If it does, then somehow the child was alive but the parent was not.
        self.parent.as_ref().map(|x| {
            x.upgrade()
                .expect("FIXME: Child process was alive for longer than the parent")
        })
    }

    pub fn new(name: String, parent: Option<Arc<Self>>) -> EResult<Self> {
        Self::new_with_space(name, parent, Arc::new(AddressSpace::new()))
    }

    pub fn fork(self: Arc<Self>, context: &Context) -> EResult<(Arc<Self>, Arc<Task>)> {
        let mut old_inner = self.inner.lock();
        let forked = Arc::new(Self {
            id: PID_COUNTER.fetch_add(1, Ordering::Acquire),
            name: self.name.clone(),
            parent: Some(Arc::downgrade(&self)),
            inner: SpinMutex::new(InnerProcess {
                status: ProcessState::Running,
                threads: Vec::new(),
                children: Vec::new(),
                address_space: Arc::new(old_inner.address_space.fork()?),
                root_dir: old_inner.root_dir.clone(),
                working_dir: old_inner.root_dir.clone(),
                identity: old_inner.identity.clone(),
                open_files: old_inner.open_files.clone(),
                mmap_head: old_inner.mmap_head,
            }),
        });

        // Create a heap allocated context that we can pass to the entry point.
        let mut forked_ctx = Box::new(*context);
        forked_ctx.set_return(0, 0); // User mode returns 0 for forked processes.
        let raw_ctx = Box::into_raw(forked_ctx);

        // Create the main thread.
        let forked_thread = Arc::new(Task::new(to_user_context, raw_ctx as _, 0, &forked, true)?);
        forked.inner.lock().threads.push(forked_thread.clone());
        old_inner.children.push(forked.clone());

        Ok((forked, forked_thread))
    }

    fn new_with_space(
        name: String,
        parent: Option<Arc<Self>>,
        space: Arc<AddressSpace>,
    ) -> EResult<Self> {
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

        // Save the child in the parent process.
        if let Some(x) = &parent {
            x.inner.lock().children.push(x.clone())
        }

        Ok(Self {
            id: PID_COUNTER.fetch_add(1, Ordering::Relaxed),
            name,
            parent: parent.map(|x| Arc::downgrade(&x)),
            inner: SpinMutex::new(InnerProcess {
                status: ProcessState::Running,
                threads: Vec::new(),
                children: Vec::new(),
                address_space: space,
                root_dir: root,
                working_dir: cwd,
                identity,
                open_files: BTreeMap::new(),
                // TODO: This address should be determined from the highest loaded segment.
                mmap_head: VirtAddr::new(0x1_0000_0000),
            }),
        })
    }

    /// Returns the kernel process.
    pub fn get_kernel() -> &'static Arc<Self> {
        KERNEL_PROCESS.get()
    }

    /// Replaces a process with a new executable image, given some arguments and an environment.
    /// The given file must be opened with ReadOnly and Executable.
    /// Any existing threads of the current process are destroyed upon a successful execve.
    /// This also means that a successful execve will never return.
    pub fn fexecve(
        self: Arc<Self>,
        file: Arc<File>,
        argv: Vec<Vec<u8>>,
        envp: Vec<Vec<u8>>,
    ) -> EResult<()> {
        let mut info = ExecInfo {
            executable: file.clone(),
            interpreter: None,
            space: AddressSpace::new(),
            argv,
            envp,
        };

        let format = vfs::exec::identify(&file).ok_or(Errno::ENOEXEC)?;
        let init = Arc::try_new(format.load(&self, &mut info)?)?;

        // If we get here, then the loading of the executable was successful.
        {
            let mut inner = self.inner.lock();
            inner.threads.clear();
            inner.threads.push(init.clone());
            inner.address_space = Arc::try_new(info.space)?;
        }

        CpuData::get().scheduler.add_task(init);

        Ok(())
    }

    pub fn exit(self: Arc<Self>, code: u8) {
        let mut inner = self.inner.lock();

        // Kill all threads.
        for thread in inner.threads.iter() {
            let mut thread_inner = thread.inner.lock();
            thread_inner.state = task::TaskState::Dead;
        }
        inner.threads.clear();

        // Close all files.
        let fds = inner.open_files.keys().cloned().collect::<Vec<_>>();
        for fd in fds {
            let desc = inner.open_files.remove(&fd);
            if let Some(desc) = desc
                && Arc::strong_count(&desc.file) == 1
            {
                _ = desc.file.close();
            }
        }
        inner.open_files.clear();

        inner.status = ProcessState::Exited(code);
        drop(inner);
        Scheduler::kill_current();
    }
}

impl InnerProcess {
    /// Attempts to get the file corresponding to the given file descriptor.
    /// Note that this does not handle special FDs like [`uapi::AT_FDCWD`].
    pub fn get_fd(&self, fd: usize) -> Option<FileDescription> {
        self.open_files.get(&fd).cloned()
    }

    /// Allocates a new descriptor for a file. Returns [`None`] if there are no more free FDs for this process.
    pub fn open_file(&mut self, file: FileDescription, base: usize) -> Option<usize> {
        // TODO: OPEN_MAX
        // Find a free descriptor.
        let mut last = base;
        loop {
            if !self.open_files.contains_key(&last) {
                break;
            }
            last += 1;
        }

        self.open_files.insert(last, file);
        Some(last)
    }
}

/// Entry point for tasks wanting to jump to user space.
pub extern "C" fn to_user(ip: usize, sp: usize) {
    unsafe { crate::arch::sched::jump_to_user(VirtAddr::from(ip), VirtAddr::from(sp)) };
}

/// Entry point for tasks wanting to jump to user space.
pub extern "C" fn to_user_context(context: usize, _: usize) {
    unsafe {
        let ctx: Box<Context> = Box::from_raw(context as _);
        let mut stack_ctx = Box::into_inner(ctx);
        crate::arch::sched::jump_to_user_context(&raw mut stack_ctx)
    };
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

#[initgraph::task(
    name = "generic.process",
    depends = [crate::memory::MEMORY_STAGE, crate::vfs::VFS_STAGE],
)]
pub fn PROCESS_STAGE() {
    // Create the kernel process and task.
    unsafe {
        KERNEL_PROCESS.init(Arc::new(
            Process::new_with_space(
                "kernel".into(),
                None,
                Arc::new(AddressSpace {
                    table: super::memory::virt::KERNEL_PAGE_TABLE.get().clone(),
                    mappings: SpinMutex::new(BTreeSet::new()),
                }),
            )
            .expect("Unable to create the main kernel process"),
        ))
    };
}
