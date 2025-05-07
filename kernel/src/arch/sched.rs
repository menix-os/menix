use super::internal;
use crate::generic::sched::task::{Frame, Task};

pub use internal::sched::Context;
assert_trait_impl!(Context, Frame);
assert_trait_impl!(Context, Default);
assert_trait_impl!(Context, Clone);
assert_trait_impl!(Context, Copy);

/// Returns the current task running on this CPU.
/// # Note
/// The implementation of this function must be an atomic operation for this to be memory safe!
pub fn get_task() -> *mut Task {
    internal::sched::get_task()
}

/// Immediately causes a reschedule event.
pub fn reschedule_now() {
    internal::sched::reschedule_now();
}

/// Switches the current CPU context from one task to another.
pub fn switch(from: *mut Task, to: *mut Task) {
    internal::sched::switch(from, to);
}
