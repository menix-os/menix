use super::task::{Task, Tid};
use crate::{
    arch::{self},
    generic::{
        percpu::CpuData,
        process::{Process, task::TaskState},
        util::mutex::Mutex,
    },
};
use alloc::{collections::btree_map::BTreeMap, sync::Arc};
use core::{
    mem,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

/// An instance of a scheduler. Each CPU has one instance running to coordinate task management.
#[derive(Debug)]
pub struct Scheduler {
    /// The currently running task on this scheduler instance. Use [`Self::get_current`] instead.
    pub(crate) current: AtomicPtr<Task>,
    pub(crate) preempt_level: usize,
    run_queue: Mutex<BTreeMap<Tid, Arc<Task>>>,
}

impl Scheduler {
    pub(crate) const fn new() -> Self {
        return Self {
            current: AtomicPtr::new(null_mut()),
            preempt_level: 0,
            run_queue: Mutex::new(BTreeMap::new()),
        };
    }

    /// Adds a task to a run queue.
    /// The scheduler will find the most optimal CPU to run on.
    pub fn add_task(&self, task: Arc<Task>) {
        self.run_queue.lock().insert(task.get_id(), task);
    }

    /// Returns the task currently running on this CPU.
    pub fn get_current() -> Arc<Task> {
        unsafe {
            let ptr = arch::sched::get_task();
            debug_assert!(!ptr.is_null());
            let task = Arc::from_raw(ptr);
            let result = task.clone();
            mem::forget(task);

            result
        }
    }

    /// Attempts to find a task by its ID on this scheduler.
    pub fn get_by_tid(&self, tid: Tid) -> Option<Arc<Task>> {
        self.run_queue.lock().get(&tid).map(|x| x.clone())
    }

    fn next(&self) -> Option<Arc<Task>> {
        let current_tid = Self::get_current().get_id();
        let filter = |&(_, b): &(&Tid, &Arc<Task>)| *b.state.lock() == TaskState::Ready;

        let rq = self.run_queue.lock();

        rq.range((current_tid + 1)..)
            .find(filter)
            .or_else(|| rq.range(..=current_tid).find(filter))
            .map(|(_, task)| task.clone())
    }

    /// Runs the scheduler. `preempt` tells the scheduler if it's supposed to handle preemption or not.
    pub(crate) fn reschedule(&self) {
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
    let task = Scheduler::get_current();
    let mut state = task.state.lock();
    *state = TaskState::Dead;
    drop(state);

    unsafe {
        arch::sched::force_reschedule();
    }

    unreachable!("The scheduler did not kill this task");
}

/// Function used for waiting.
pub extern "C" fn idle_fn(_: usize, _: usize) {
    unsafe { crate::arch::irq::set_irq_state(true) };
    loop {
        crate::arch::irq::wait_for_irq();
    }
}

init_stage! {
    #[depends(crate::generic::memory::MEMORY_STAGE, super::PROCESS_STAGE)]
    pub SCHEDULER_STAGE: "generic.scheduler" => init;
}

fn init() {
    // Set up scheduler.
    let bsp_scheduler = &CpuData::get().scheduler;
    let initial = Arc::new(Task::new(idle_fn, 0, 0, Process::get_kernel(), false).unwrap());
    bsp_scheduler.add_task(initial.clone());

    let to = Arc::into_raw(initial);
    bsp_scheduler.current.store(to as *mut _, Ordering::Relaxed);
}
