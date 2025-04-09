use crate::arch::irq::InterruptFrame;
use alloc::sync::Arc;
use core::sync::atomic::AtomicUsize;
use thread::Thread;

pub mod process;
pub mod thread;

/// An instance of a scheduler. Each CPU has one instance running to coordinate thead management.
pub struct Scheduler {
    /// Determines if this scheduler should be allowed to preempt the currently running task.
    do_preempt: bool,
    /// Amount of ticks the current thread has been running for.
    ticks_active: AtomicUsize,
    /// The active thread on this scheduler instance.
    thread: Option<Arc<Thread>>,
}

impl Scheduler {
    /// Enables preemption on the current core.
    pub fn preempt_on(&mut self) {
        self.do_preempt = false
    }

    /// Disables preemption on the current core.
    pub fn preempt_off(&mut self) {
        self.do_preempt = true
    }

    pub fn get_current_thread(&self) -> Option<Arc<Thread>> {
        match &self.thread {
            Some(x) => return Some(x.clone()),
            None => None,
        }
    }

    pub fn reschedule<'a>(&mut self, mut context: &'a InterruptFrame) -> &'a InterruptFrame {
        self.preempt_off();
        // TODO: Reschedule
        self.preempt_on();
        return context;
    }
}
