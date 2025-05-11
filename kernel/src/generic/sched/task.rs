use super::process::{Pid, Process};
use crate::arch::sched::Context;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Copy, Debug)]
pub enum TaskState {
    /// Ready to run.
    Ready,
    /// Currently being executed.
    Running,
    /// Waiting for a timer or another signal.
    Waiting,
    /// Task is killed and waiting for cleanup.
    Dead,
}

pub type Tid = usize;

/// Represents the atomic scheduling structure.
#[derive(Debug)]
pub struct Task {
    /// The unique identifier of this task.
    pub id: Tid,
    /// The saved context of a task while it is not running. This field is architecture specific.
    pub context: Context,
    /// The current state of the thread.
    pub state: TaskState,
    /// The process which this task belongs to. A value of [`None`] indicates a kernel task.
    pub parent: Option<Pid>,
}

impl Task {
    pub fn new() -> Self {
        return Self {
            id: TASK_ID_COUNTER.fetch_add(1, Ordering::Acquire),
            context: Context::default(),
            state: TaskState::Ready,
            parent: None,
        };
    }

    /// Creates a new task as a thread for a process.
    pub fn new_thread(proc: Process) -> Self {
        return Self {
            id: TASK_ID_COUNTER.fetch_add(1, Ordering::Acquire),
            context: Context::default(),
            state: TaskState::Ready,
            parent: Some(proc.get_pid()),
        };
    }
}

/// Global counter to provide new task IDs.
static TASK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
