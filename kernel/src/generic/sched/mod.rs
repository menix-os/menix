use crate::arch::{self};
use core::{
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};
use task::Task;

use super::util::spin::SpinLock;

pub mod process;
pub mod task;

/// An instance of a scheduler. Each CPU has one instance running to coordinate thread management.
#[derive(Debug)]
pub struct Scheduler {
    /// The currently running task on this scheduler instance.
    pub(crate) current: AtomicPtr<Task>,
    pub(crate) lock: SpinLock,
    ticks_active: usize,
    preempt_level: usize,
    preempt_queued: bool,
}

impl Scheduler {
    pub const fn uninit() -> Self {
        return Self {
            current: AtomicPtr::new(null_mut()),
            lock: SpinLock::new(),
            ticks_active: 0,
            preempt_level: 0,
            preempt_queued: false,
        };
    }

    /// Runs the scheduler. `preempt` tells the scheduler if it's supposed to handle preemption or not.
    /// # Safety
    /// Do not call this directly!
    pub(crate) unsafe fn tick(&mut self, preempt: bool) {
        // Disable interrupts.

        let from = self.current.load(Ordering::Relaxed);
        // TODO
        let to = self.current.load(Ordering::Relaxed);

        // Enable interrupts.
        unsafe { arch::irq::set_irq_state(true) };

        arch::sched::switch(from, to);
    }

    /// Starts executing this scheduler.
    pub(crate) fn start(&mut self) {
        unsafe { arch::irq::set_irq_state(true) };
    }
}
