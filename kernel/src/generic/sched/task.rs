use super::process::{Pid, Process};
use crate::{
    arch::{self},
    generic::{
        errno::Errno,
        memory::{VirtAddr, virt::KERNEL_STACK_SIZE},
        util::mutex::Mutex,
    },
};
use core::{
    alloc::Layout,
    panic,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TaskState {
    /// Currently being executed.
    Running,
    /// Ready to run.
    Ready,
    /// Waiting for a timer or another signal.
    Waiting,
    /// Task is being killed.
    Dying,
    /// Task is killed and waiting for cleanup.
    Dead,
}

pub type Tid = usize;

/// Represents the atomic scheduling structure.
#[derive(Debug)]
pub struct Task {
    /// A pointer to the saved context of user mode registers.
    pub user_context: Option<NonNull<arch::sched::Context>>,
    /// The saved context of a task while it is not running.
    pub task_context: Mutex<arch::sched::TaskContext>,
    /// The kernel stack for this task.
    // TODO: Use kernel stack structure that handles memory management.
    pub stack: VirtAddr,
    /// The unique identifier of this task.
    id: Tid,
    /// The current state of the thread.
    state: TaskState,
    /// The process which this task belongs to.
    process: Option<Pid>,
    /// If this task is a user task. `false` forbids this task to ever enter user mode.
    is_user: bool,
}

impl Task {
    /// Creates a new task.
    pub fn new(
        entry: extern "C" fn(usize) -> !,
        arg: usize,
        parent: Option<&Process>,
        is_user: bool,
    ) -> Result<Self, Errno> {
        // TODO: see above
        const STACK_LAYOUT: Layout = match Layout::from_size_align(KERNEL_STACK_SIZE, 0x1000) {
            Ok(x) => x,
            Err(_) => panic!("Layout error"),
        };

        let result = Self {
            id: TASK_ID_COUNTER.fetch_add(1, Ordering::Acquire),
            user_context: None,
            task_context: Mutex::new(arch::sched::TaskContext::default()),
            state: TaskState::Ready,
            process: parent.map(|x| x.get_pid()),
            is_user,
            stack: unsafe { alloc::alloc::alloc_zeroed(STACK_LAYOUT).into() },
        };

        arch::sched::init_task(
            &mut result.task_context.lock(),
            entry,
            arg,
            result.stack,
            is_user,
        )?;

        return Ok(result);
    }

    /// Returns true if this is a user task.
    #[inline]
    pub const fn is_user(&self) -> bool {
        self.is_user
    }

    /// Returns the ID of this task.
    #[inline]
    pub const fn get_id(&self) -> Tid {
        self.id
    }

    /// Returns the process which this task belongs to. If it doesn't belong to any, [`None`] is returned.
    #[inline]
    pub const fn get_process(&self) -> Option<Pid> {
        self.process
    }

    pub fn is_ready(&self) -> bool {
        self.state == TaskState::Ready
    }
}

/// Global counter to provide new task IDs.
static TASK_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
