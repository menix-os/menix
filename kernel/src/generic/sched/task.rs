use super::process::{Pid, Process};
use crate::{arch, generic::memory::virt::KERNEL_STACK_SIZE};
use alloc::{boxed::Box, sync::Arc};
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Copy, Debug)]
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
    /// The saved context of user mode registers.
    pub user_context: arch::sched::Context,
    /// The saved context of a task while it is not running.
    pub task_context: arch::sched::TaskContext,
    /// The kernel stack for this task.
    pub stack: Box<[u8]>,
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
    ) -> Arc<Self> {
        let mut result = Self {
            id: TASK_ID_COUNTER.fetch_add(1, Ordering::Acquire),
            user_context: arch::sched::Context::default(),
            task_context: arch::sched::TaskContext::default(),
            state: TaskState::Ready,
            process: parent.map(|x| x.get_pid()),
            is_user,
            stack: unsafe { Box::new_zeroed_slice(KERNEL_STACK_SIZE).assume_init() },
        };
        arch::sched::init_task(
            &mut result.task_context,
            entry,
            arg,
            result.stack.as_ptr() as usize,
            is_user,
        );

        return Arc::new(result);
    }

    /// Returns true if this is a user task.
    #[inline]
    pub const fn is_user(&self) -> bool {
        self.is_user
    }
}

/// Global counter to provide new task IDs.
static TASK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
