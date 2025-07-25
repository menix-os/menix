use super::Process;
use crate::{
    arch::{self},
    generic::{memory::virt::KERNEL_STACK_SIZE, posix::errno::EResult, util::mutex::Mutex},
};
use alloc::sync::{Arc, Weak};
use core::{
    alloc::Layout,
    panic,
    ptr::NonNull,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
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
    /// The unique identifier of this task.
    id: Tid,
    /// The process which this task belongs to.
    process: Weak<Process>,
    /// If this task is a user task. `false` forbids this task to ever enter user mode.
    is_user: bool,
    /// The current state of the thread.
    pub state: Mutex<TaskState>,
    /// A pointer to the saved context of user mode registers.
    pub user_context: Option<NonNull<arch::sched::Context>>,
    /// The saved context of a task while it is not running.
    pub task_context: Mutex<arch::sched::TaskContext>,
    /// The kernel stack for this task.
    pub kernel_stack: AtomicUsize,
    /// The kernel stack for this task.
    pub user_stack: AtomicUsize,
    /// The amount of time that this task can live on.
    pub ticks: usize,
    /// A value between -20 and 19, where -20 is the highest priority and 0 is a neutral priority.
    pub priority: i8,
    /// Whether the current task is in the process of being execve'd.
    pub in_execve: AtomicBool,
}

impl Task {
    /// Creates a new task.
    pub fn new(
        entry: extern "C" fn(usize, usize),
        arg1: usize,
        arg2: usize,
        parent: &Arc<Process>,
        is_user: bool,
    ) -> EResult<Self> {
        // TODO: see above
        const STACK_LAYOUT: Layout = match Layout::from_size_align(KERNEL_STACK_SIZE, 0x1000) {
            Ok(x) => x,
            Err(_) => panic!("Layout error"),
        };

        let result = Self {
            user_context: None,
            task_context: Mutex::new(arch::sched::TaskContext::default()),
            ticks: 0,
            priority: 0,
            kernel_stack: AtomicUsize::new(
                unsafe { alloc::alloc::alloc_zeroed(STACK_LAYOUT) } as usize
            ),
            user_stack: AtomicUsize::new(0),
            id: TASK_ID_COUNTER.fetch_add(1, Ordering::Acquire),
            state: Mutex::new(TaskState::Ready),
            process: Arc::downgrade(parent),
            is_user,
            in_execve: AtomicBool::new(false),
        };

        arch::sched::init_task(
            &mut result.task_context.lock(),
            entry,
            arg1,
            arg2,
            result.kernel_stack.load(Ordering::Acquire).into(),
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

    /// Returns the process which this task belongs to.
    #[inline]
    pub fn get_process(&self) -> Arc<Process> {
        if let Some(x) = self.process.upgrade() {
            x
        } else {
            todo!()
        }
    }
}

/// Global counter to provide new task IDs.
static TASK_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
