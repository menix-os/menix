pub mod process;
pub mod task;

use super::util::mutex::IrqMutex;
use crate::{
    arch,
    generic::{percpu::CpuData, sched::task::TaskState},
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
    /// The currently running task on this scheduler instance. Use [`Self::get_current`] instead.
    pub(crate) current: AtomicPtr<Task>,
    pub(crate) lock: IrqMutex<()>,
    pub(crate) preempt_level: usize,
    run_queue: BTreeMap<Tid, Arc<Task>>,
}

impl Scheduler {
    pub(crate) const fn new() -> Self {
        return Self {
            current: AtomicPtr::new(null_mut()),
            lock: IrqMutex::new(()),
            preempt_level: 0,
            run_queue: BTreeMap::new(),
        };
    }

    pub fn add_task(&mut self, task: Task) {
        log!("New task {} added to run queue", task.get_id());
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

        // We create a dummy thread on the stack which only exists to start the scheduler since
        // the scheduler assumes that there's always a task running. Since this is a dead end,
        // we don't actually add this to the run queue.
        let dummy = Task::new(dummy_fn, 0, 0, None, false).unwrap();

        unsafe {
            let to = Arc::into_raw(task);
            self.current.store(to as *mut _, Ordering::Relaxed);

            arch::sched::switch(&raw const dummy, to);
        }

        unreachable!("Failed to start scheduling!");
    }

    fn next(&self) -> Option<Arc<Task>> {
        let current_tid = Self::get_current().get_id();
        let filter = |&(_, b): &(&Tid, &Arc<Task>)| *b.state.lock() == TaskState::Ready;

        self.run_queue
            .range((current_tid + 1)..)
            .find(filter)
            .or_else(|| self.run_queue.range(..=current_tid).find(filter))
            .map(|(_, task)| task.clone())
    }

    /// Runs the scheduler. `preempt` tells the scheduler if it's supposed to handle preemption or not.
    pub(crate) fn reschedule(&mut self) {
        let old = unsafe { arch::irq::set_irq_state(false) };
        let from = self.current.load(Ordering::Relaxed);
        let to = Arc::into_raw(self.next().expect("No more tasks to run!")) as *mut Task;

        if from == to {
            unsafe { arch::irq::set_irq_state(old) };
            return;
        }

        self.current.store(to, Ordering::Relaxed);

        unsafe {
            arch::sched::switch(from, to);
            arch::irq::set_irq_state(old);
        }
    }
}

/// Generic task entry point. This is to be called by an implementing [`crate::arch::sched::init_task`].
pub extern "C" fn task_entry(entry: extern "C" fn(usize, usize), arg1: usize, arg2: usize) -> ! {
    (entry)(arg1, arg2);

    // The task function is over, kill the task.
    {
        let task = Scheduler::get_current();
        *task.state.lock() = TaskState::Dead;
    }

    CpuData::get().scheduler.reschedule();
    unreachable!();
}

extern "C" fn dummy_fn(_: usize, _: usize) {
    unreachable!("This is a dummy function, somehow the dummy task ended up in the scheduler");
}
