use crate::arch::virt::TaskFrame;

use super::Task;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// An instance of a scheduler. Each CPU has one instance running to coordinate thead management.
#[derive(Debug)]
pub struct Scheduler {
    /// Determines if this scheduler should be allowed to preempt the currently running task.
    do_preempt: AtomicBool,
    /// Amount of ticks the current thread has been running for.
    ticks_active: AtomicUsize,
    /// The active task on this scheduler instance.
    task: Option<Arc<Task>>,
}

static SCHEDULER: Scheduler = Scheduler::new();

impl Scheduler {
    pub const fn new() -> Self {
        return Self {
            do_preempt: AtomicBool::new(false),
            ticks_active: AtomicUsize::new(0),
            task: None,
        };
    }
}

/// Enables preemption on the current core.
pub fn preempt_on() {
    SCHEDULER.do_preempt.store(false, Ordering::Release);
}

/// Disables preemption on the current core.
pub fn preempt_off() {
    SCHEDULER.do_preempt.store(true, Ordering::Release);
}

pub fn get_current_task() -> Option<Arc<Task>> {
    match &SCHEDULER.task {
        Some(x) => return Some(x.clone()),
        None => None,
    }
}

pub fn reschedule(mut context: &TaskFrame) -> &TaskFrame {
    preempt_off();
    // TODO: Reschedule
    preempt_on();
    return context;
}
