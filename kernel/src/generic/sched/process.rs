use crate::generic::memory::{
    pmm::FreeList,
    virt::{KERNEL_PAGE_TABLE, PageTable},
};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

use super::task::Task;

pub type Pid = usize;

/// Represents a user process and address space.
pub struct Process {
    /// The unique identifier of this process.
    id: Pid,
    page_table: PageTable,
    threads: Vec<Task>,
}

static PID_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Process {
    pub fn new() -> Self {
        Self {
            id: PID_COUNTER.fetch_add(1, Ordering::Relaxed),
            page_table: PageTable::new_user::<FreeList>(KERNEL_PAGE_TABLE.lock().root_level()),
            threads: Vec::new(),
        }
    }

    /// Returns the unique identifier of this process.
    #[inline]
    pub const fn get_pid(&self) -> Pid {
        self.id
    }
}
