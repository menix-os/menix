use alloc::boxed::Box;

use crate::arch::{Context, PageTableEntry};

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

/// Represents the atomic scheduling structure.
pub struct Task {
    next: Option<Box<Task>>,
    /// The saved context of a thread while the thread is not running.
    context: Context,
    /// The current state of the thread.
    state: TaskState,
}

impl Task {
    pub fn new() -> Self {
        return Self {
            next: None,
            context: Context::default(),
            state: TaskState::Ready,
        };
    }
}
