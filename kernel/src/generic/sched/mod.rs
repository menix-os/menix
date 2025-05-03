use crate::arch::{self, exec::TaskFrame};
use core::{
    ptr::null_mut,
    sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering},
};
use task::Task;

pub mod task;

/// An instance of a scheduler. Each CPU has one instance running to coordinate thread management.
#[derive(Debug)]
pub struct Scheduler {
    /// Determines if this scheduler should be allowed to preempt the currently running task.
    do_preempt: AtomicBool,
    /// Amount of ticks the current thread has been running for.
    ticks_active: AtomicUsize,
    /// The active task on this scheduler instance.
    pub task: AtomicPtr<Task>,
}

impl Scheduler {
    pub const fn uninit() -> Self {
        return Self {
            do_preempt: AtomicBool::new(false),
            ticks_active: AtomicUsize::new(0),
            task: AtomicPtr::new(null_mut()),
        };
    }

    /// Enables preemption on the current core.
    pub fn preempt_on(&mut self) {
        self.do_preempt.store(false, Ordering::Release);
    }

    /// Disables preemption on the current core.
    pub fn preempt_off(&mut self) {
        self.do_preempt.store(true, Ordering::Release);
    }

    pub fn reschedule<'a>(&mut self, context: &'a TaskFrame) -> &'a TaskFrame {
        // Disable interrupts.
        unsafe { arch::irq::set_irq_state(false) };

        // TODO

        // Enable interrupts.
        unsafe { arch::irq::set_irq_state(true) };
        return context;
    }
}
