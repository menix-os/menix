use crate::arch::exec::TaskFrame;
use core::sync::atomic::{AtomicUsize, Ordering};

pub trait Frame {
    fn set_stack(&mut self, addr: usize);
    fn get_stack(&self) -> usize;

    fn set_ip(&mut self, addr: usize);
    fn get_ip(&self) -> usize;

    /// Saves a copy of this frame.
    fn save(&self) -> TaskFrame;
    /// Reconstructs itself from a saved frame.
    fn restore(&mut self, saved: TaskFrame);
}

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

/// Represents the atomic scheduling structure.
#[derive(Debug)]
pub struct Task {
    /// Unique identifier
    pub id: usize,
    /// The saved context of a thread while the thread is not running.
    pub context: TaskFrame,
    /// The current state of the thread.
    pub state: TaskState,
}

/// Global counter to provide new task IDs.
static TASK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Task {
    pub fn new() -> Self {
        return Self {
            id: TASK_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            context: TaskFrame::new(),
            state: TaskState::Ready,
        };
    }

    pub fn with_entry(mut self, entry_point: usize, stack: usize) -> Self {
        self.context.set_ip(entry_point);
        self.context.set_stack(stack);
        return self;
    }
}
