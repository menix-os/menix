/// An instance of a scheduler. Each CPU has one instance running to coordinate thead management.
pub struct Scheduler {
    /// Determines if this scheduler should be allowed to preempt the currently running task.
    do_preempt: bool,
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
}
