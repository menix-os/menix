use crate::{
    arch::irq::InterruptFrame,
    generic::memory::{VirtAddr, virt::KERNEL_STACK_SIZE},
};
use alloc::{sync::Arc, vec};
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

/// Represents the atomic scheduling structure.
#[derive(Debug)]
pub struct Task {
    next: Option<Arc<Task>>,
    /// Unique identifier
    pub id: usize,
    /// The saved context of a thread while the thread is not running.
    pub context: InterruptFrame,
    /// The current state of the thread.
    pub state: TaskState,
}

/// Global counter to provide new task IDs.
static TASK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Task {
    pub fn new() -> Self {
        return Self {
            next: None,
            id: TASK_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            context: InterruptFrame::new(),
            state: TaskState::Ready,
        };
    }

    pub const fn with_entry(mut self, entry_point: VirtAddr, stack: VirtAddr) -> Self {
        self.context.set_ip(entry_point);
        self.context.set_stack(stack);
        return self;
    }
}
