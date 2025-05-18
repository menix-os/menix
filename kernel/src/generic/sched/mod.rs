pub mod process;
pub mod task;

use super::util::spin::SpinLock;
use crate::{
    arch::{self},
    generic::irq,
};
use alloc::{collections::btree_map::BTreeMap, sync::Arc};
use core::{
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};
use task::{Task, Tid};

/// An instance of a scheduler. Each CPU has one instance running to coordinate thread management.
#[derive(Debug)]
pub struct Scheduler {
    /// The currently running task on this scheduler instance.
    pub(crate) current: AtomicPtr<Task>,
    pub(crate) lock: SpinLock,
    ticks_active: usize,
    preempt_level: usize,
    preempt_queued: bool,
    run_queue: BTreeMap<Tid, Arc<Task>>,
}

impl Scheduler {
    pub(crate) const fn uninit() -> Self {
        return Self {
            current: AtomicPtr::new(null_mut()),
            lock: SpinLock::new(),
            ticks_active: 0,
            preempt_level: 0,
            preempt_queued: false,
            run_queue: BTreeMap::new(),
        };
    }

    pub fn add_task(&mut self, task: Task) {
        self.run_queue.insert(task.get_id(), Arc::new(task));
    }

    /// Returns the task currently running on this CPU.
    pub fn get_current() -> Arc<Task> {
        unsafe {
            let ptr = arch::sched::get_task();
            Arc::from_raw(ptr).clone()
        }
    }

    pub fn get_by_tid(&self, tid: Tid) -> Option<Arc<Task>> {
        self.run_queue.get(&tid).map(|x| x.clone())
    }

    /// Starts running this scheduler.
    pub(crate) fn start(&mut self, initial: Task) -> ! {
        let task = Arc::new(initial);

        self.run_queue.insert(task.get_id(), task.clone());
        self.current
            .store(Arc::into_raw(task) as *mut _, Ordering::Release);
        unsafe { arch::irq::set_irq_state(true) };
        unsafe { arch::sched::force_reschedule() };

        unreachable!("Failed to start scheduling!");
    }

    fn next(&self) -> Option<Arc<Task>> {
        let current_tid = Self::get_current().get_id();

        let filter = |&(_, b): &(&Tid, &Arc<Task>)| b.is_ready();

        self.run_queue
            .range((current_tid + 1)..)
            .find(filter)
            .or_else(|| self.run_queue.range(..=current_tid).find(filter))
            .map(|(_, task)| task.clone())
    }

    /// Runs the scheduler. `preempt` tells the scheduler if it's supposed to handle preemption or not.
    /// Returns a tuple of (old task, new task).
    /// # Safety
    /// The returned value *must* be used by the caller to finish the task switch with [`Self::finish_reschedule`].
    pub(crate) unsafe fn start_reschedule(&mut self, preempt: bool) -> (*const Task, *const Task) {
        let from = self.current.load(Ordering::Relaxed);
        let to = Arc::into_raw(self.next().unwrap());
        self.current.store(to as *mut Task, Ordering::Relaxed);
        return (from, to);
    }

    pub(crate) unsafe fn finish_reschedule(&mut self, from: *const Task, to: *const Task) {
        unsafe { arch::sched::switch(from, to) };
    }
}
