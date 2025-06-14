use super::task::{Task, Tid};
use crate::{
    arch::{self},
    generic::{
        percpu::CpuData,
        process::{Process, task::TaskState},
        util::mutex::Mutex,
    },
};
use alloc::{collections::btree_map::BTreeMap, string::ToString, sync::Arc};
use core::{
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
    pub fn add_task(task: Task) {
        // TODO: Find a CPU with the lowest effective load.
        let optimal = &CpuData::get().scheduler;

        optimal
            .run_queue
            .lock()
            .insert(task.get_id(), Arc::new(task));
    }

    /// Returns the task currently running on this CPU.
    pub fn get_current() -> Arc<Task> {
        unsafe {
            let ptr = arch::sched::get_task();
            Arc::from_raw(ptr).clone()
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
    {
        let task = Scheduler::get_current();
        *task.state.lock() = TaskState::Dead;
    }

    CpuData::get().scheduler.reschedule();
    unreachable!();
}

init_stage! {
    #[depends(crate::generic::memory::MEMORY_STAGE, crate::generic::vfs::VFS_STAGE)]
    pub SCHEDULER_STAGE: "generic.scheduler" => init;
}

fn init() {
    // Create the kernel process and task.
    unsafe {
        super::KERNEL_PROCESS.init(Arc::new(
            Process::new("kernel".to_string(), None, false)
                .expect("Unable to create the main kernel process"),
        ))
    };

    let task = Arc::new(
        Task::new(idle_fn, 0, 0, Process::get_kernel().clone(), false)
            .expect("Unable to create the main kernel task"),
    );

    // Set up scheduler structures on all CPUs.
    let sched = &CpuData::get().scheduler;
    sched.run_queue.lock().insert(task.get_id(), task.clone());

    // We create a dummy task on the stack which only exists to start the
    // scheduler since it assumes that there's always a task running.
    // Because this is a dead end, we don't actually add this to the run queue.
    // Note: This also stops the kernel process from being freed.
    let dummy = Task::new(dummy_fn, 0, 0, Process::get_kernel().clone(), false).unwrap();

    unsafe {
        let to = Arc::into_raw(task);
        sched.current.store(to as *mut _, Ordering::Relaxed);

        arch::sched::switch(&raw const dummy, to);
    }

    unreachable!("Failed to start scheduling");
}

/// This function is used for idling CPUs waiting to be scheduled a real job. Does essentially nothing.
extern "C" fn idle_fn(_: usize, _: usize) {
    loop {
        core::hint::spin_loop();
        arch::irq::wait_for_irq();
    }
}

extern "C" fn dummy_fn(_: usize, _: usize) {
    unreachable!("This is a dummy function, somehow the dummy task ended up in the scheduler");
}
