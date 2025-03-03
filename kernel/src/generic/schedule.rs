/// Initializes scheduling.
pub fn init() {}

/// Enables preemption.
pub fn preempt_on() {}
/// Disables preemption.
pub fn preempt_off() {}

/// This structure makes sure that preemption is turned off while the containing structure is being accessed.
/// Add a field of this type to any struct that needs this constraint.
pub struct PreemptionGuard {
    _p: (),
}

impl !Send for PreemptionGuard {}

impl Drop for PreemptionGuard {
    fn drop(&mut self) {
        preempt_on();
    }
}

impl PreemptionGuard {
    pub fn get() -> PreemptionGuard {
        // assert!(preemption_is_on());
        preempt_off();
        PreemptionGuard { _p: () }
    }
}
