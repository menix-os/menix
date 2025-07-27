use crate::{
    arch::{self},
    generic::{
        percpu::{CPU_DATA, CpuData},
        process::{
            Process,
            task::{Task, TaskState},
        },
        util::spin_mutex::SpinMutex,
    },
};
use alloc::{collections::vec_deque::VecDeque, sync::Arc};
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
    pub(crate) idle_task: AtomicPtr<Task>,
    pub(crate) preempt_level: usize,
    run_queue: SpinMutex<VecDeque<Arc<Task>>>,
}

impl Scheduler {
    pub(crate) const fn new() -> Self {
        return Self {
            current: AtomicPtr::new(null_mut()),
            idle_task: AtomicPtr::new(null_mut()),
            preempt_level: 0,
            run_queue: SpinMutex::new(VecDeque::new()),
        };
    }

    /// Returns a reference to the idle task.
    fn idle_task(&self) -> *mut Task {
        let ptr = self.idle_task.load(Ordering::Relaxed);
        debug_assert!(!ptr.is_null());
        ptr
    }

    /// Adds a task to a run queue.
    pub fn add_task(&self, task: Arc<Task>) {
        self.run_queue.lock().push_back(task);
    }

    /// Returns the task currently running on this CPU.
    pub fn get_current() -> Arc<Task> {
        let ptr = arch::sched::get_task();
        debug_assert!(!ptr.is_null());
        let task = unsafe { Arc::from_raw(ptr) };
        let result = task.clone();
        mem::forget(task);
        result
    }

    fn next(&self) -> Option<Arc<Task>> {
        self.run_queue.lock().pop_front()
    }

    /// Puts the current task back to the run queue and reschedules.
    pub fn reschedule(&self) {
        let old = unsafe { arch::irq::set_irq_state(false) };
        let idle = self.idle_task();
        let from = self.current.load(Ordering::Relaxed);

        if from != idle {
            self.add_task(unsafe {
                let task = Arc::from_raw(from);
                let result = task.clone();
                mem::forget(task);
                result
            });
        }

        self.do_reschedule();
        unsafe { arch::irq::set_irq_state(old) };
    }

    /// Reschedules without adding the current task back to the run queue.
    pub fn do_yield(&self) {
        let old = unsafe { arch::irq::set_irq_state(false) };
        self.do_reschedule();
        unsafe { arch::irq::set_irq_state(old) };
    }

    /// Runs the scheduler.
    fn do_reschedule(&self) {
        let from = self.current.load(Ordering::Relaxed);
        let to = self
            .next()
            .map(|task| Arc::into_raw(task) as *mut _)
            .unwrap_or(self.idle_task());

        if from == to {
            return;
        }

        self.current.store(to, Ordering::Relaxed);

        unsafe {
            // If we are switching to a task from another process, we need to update the page table.
            {
                let from_proc = (*from).get_process();
                let to_proc = (*to).get_process();
                if !Arc::ptr_eq(&from_proc, &to_proc) {
                    arch::virt::set_page_table(
                        to_proc.inner.lock().address_space.table.get_head_addr(),
                    );
                }

                let cpu = CPU_DATA.get();
                let mut from_inner = (*from).inner.lock();
                from_inner.kernel_stack = cpu.kernel_stack.load(Ordering::Acquire).into();
                from_inner.user_stack = cpu.user_stack.load(Ordering::Acquire).into();

                let to_inner = (*to).inner.lock();
                cpu.kernel_stack
                    .store(to_inner.kernel_stack.value(), Ordering::Release);
                cpu.user_stack
                    .store(to_inner.user_stack.value(), Ordering::Release);
            }

            arch::sched::switch(from, to);
        }
    }

    /// Kills the currently running task.
    pub fn kill_current() -> ! {
        let task = Scheduler::get_current();
        let mut inner = task.inner.lock();
        inner.state = TaskState::Dead;
        drop(inner);
        CPU_DATA.get().scheduler.do_yield();
        unreachable!("The scheduler did not kill this task");
    }
}

/// Generic task entry point. This is to be called by an implementing [`crate::arch::sched::init_task`].
pub extern "C" fn task_entry(entry: extern "C" fn(usize, usize), arg1: usize, arg2: usize) -> ! {
    (entry)(arg1, arg2);

    // The task function is over, kill the task.
    Scheduler::kill_current();
}

/// Function used for waiting.
pub extern "C" fn idle_fn(_: usize, _: usize) {
    unsafe { crate::arch::irq::set_irq_state(true) };
    loop {
        crate::arch::irq::wait_for_irq();
    }
}

#[initgraph::task(
    name = "generic.scheduler",
    depends = [crate::generic::memory::MEMORY_STAGE, super::process::PROCESS_STAGE],
)]
pub fn SCHEDULER_STAGE() {
    // Set up scheduler.
    let bsp_scheduler = &CpuData::get().scheduler;
    let idle_task = Arc::new(Task::new(idle_fn, 0, 0, &Process::get_kernel(), false).unwrap());
    let initial_task =
        Arc::new(Task::new(crate::main, 0, 0, &Process::get_kernel(), false).unwrap());

    bsp_scheduler.add_task(initial_task);

    let idle_task_ptr = Arc::into_raw(idle_task);
    bsp_scheduler
        .current
        .store(idle_task_ptr as *mut _, Ordering::Relaxed);
    bsp_scheduler
        .idle_task
        .store(idle_task_ptr as *mut _, Ordering::Relaxed);
}
