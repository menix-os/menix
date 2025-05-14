use super::process::{Pid, Process};
use crate::arch;
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
    /// The unique identifier of this task.
    id: Tid,
    /// The saved context of a task while it is not running.
    pub context: arch::sched::Context,
    /// The saved context of a task while it is not running.
    pub task_context: arch::sched::TaskContext,
    /// The current state of the thread.
    state: TaskState,
    /// The process which this task belongs to.
    parent: Pid,
    /// If this task is a user task. `false` forbids this task to ever enter user mode.
    is_user: bool,
}

impl Task {
    pub fn new(is_user: bool) -> Self {
        return Self {
            id: TASK_ID_COUNTER.fetch_add(1, Ordering::Acquire),
            context: arch::sched::Context::default(),
            task_context: arch::sched::TaskContext::default(),
            state: TaskState::Ready,
            parent: 0,
            is_user,
        };
    }

    /// Creates a new task as a thread for a process.
    pub fn new_thread(proc: &Process) -> Self {
        return Self {
            id: TASK_ID_COUNTER.fetch_add(1, Ordering::Acquire),
            context: arch::sched::Context::default(),
            task_context: arch::sched::TaskContext::default(),
            state: TaskState::Ready,
            parent: proc.get_pid(),
            is_user: proc.is_user(),
        };
    }

    /// Returns true if this is a user process.
    #[inline]
    pub const fn is_user(&self) -> bool {
        self.is_user
    }
}

/// Global counter to provide new task IDs.
static TASK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
