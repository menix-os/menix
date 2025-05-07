use super::internal;
use crate::generic::sched::task::{Frame, Task};
use alloc::sync::Arc;

pub use internal::sched::Context;
assert_trait_impl!(Context, Frame);
assert_trait_impl!(Context, Default);

/// Returns the current task running on this CPU.
/// # Note
/// The implementation of this function must be an atomic operation for this to be memory safe!
pub fn get_task() -> Arc<Task> {
    unsafe { Arc::from_raw(internal::core::get_task()) }
}

pub fn switch_task(old: &Context, new: &mut Context) {}
