use crate::arch::{schedule::Context, virt::PageTableEntry};
use alloc::{boxed::Box, rc::Rc, sync::Arc};
use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use spin::{Mutex, RwLock};

use super::virt::PageTable;

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
    /// The corresponding page table.
    pub page_table: Arc<RwLock<PageTable>>,
    /// The saved context of a thread while the thread is not running.
    pub context: Context,
    /// The current state of the thread.
    pub state: TaskState,
}

/// Global counter to provide new task IDs.
static TASK_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Task {
    pub fn new(page_table: Arc<RwLock<PageTable>>) -> Self {
        return Self {
            id: TASK_ID_COUNTER.fetch_add(1, Relaxed),
            context: Context::default(),
            state: TaskState::Ready,
            page_table,
        };
    }
}
