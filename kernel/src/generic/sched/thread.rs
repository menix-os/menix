use crate::{arch::irq::InterruptFrame, generic::memory::VirtAddr};
use alloc::sync::Arc;
use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};

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
pub struct Thread {
    next: Option<Arc<Thread>>,
    /// Unique identifier
    pub id: usize,
    /// The saved context of a thread while the thread is not running.
    pub context: InterruptFrame,
    /// The current state of the thread.
    pub state: TaskState,
}

/// Global counter to provide new task IDs.
static THREAD_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Default for Thread {
    fn default() -> Self {
        Self::new()
    }
}

impl Thread {
    pub fn new() -> Self {
        return Self {
            next: None,
            id: THREAD_ID_COUNTER.fetch_add(1, Relaxed),
            context: InterruptFrame::default(),
            state: TaskState::Ready,
        };
    }

    /// Creates a new thread ready for execution.
    pub fn new_exec(entry_point: VirtAddr, stack: VirtAddr) -> Self {
        let mut result = Self::new();
        result.context.set_ip(entry_point);
        result.context.set_stack(stack);
        return result;
    }
}
