use super::internal;
use crate::generic::memory::VirtAddr;
use crate::generic::posix::errno::EResult;
use crate::generic::sched::task::Task;

pub use internal::sched::Context;
assert_trait_impl!(Context, Default);
assert_trait_impl!(Context, Clone);
assert_trait_impl!(Context, Copy);

pub use internal::sched::TaskContext;
assert_trait_impl!(TaskContext, Default);
assert_trait_impl!(TaskContext, Clone);
assert_trait_impl!(TaskContext, Copy);

/// Returns the current task running on this CPU.
/// # Safety
/// The implementation of this function must be an atomic operation for this to be memory safe!
#[inline]
pub fn get_task() -> *const Task {
    internal::sched::get_task()
}

/// Disables preemption.
/// # Safety
/// The implementation of this function must be an atomic operation for this to be memory safe!
#[inline]
pub unsafe fn preempt_disable() {
    unsafe { internal::sched::preempt_disable() };
}

/// Enables preemption. Returns true, if a reschedule was queued.
/// # Safety
/// The implementation of this function must be an atomic operation for this to be memory safe!
#[inline]
pub unsafe fn preempt_enable() -> bool {
    unsafe { internal::sched::preempt_enable() }
}

/// Switches the current CPU context from one task to another.
pub unsafe fn switch(from: *const Task, to: *const Task) {
    unsafe { internal::sched::switch(from, to) }
}

/// Forces a rescheduling interrupt.
/// # Safety
/// Rescheduling must be safe at the point of this call.
pub unsafe fn force_reschedule() {
    unsafe { internal::sched::force_reschedule() }
}

/// Initializes a new task.
pub fn init_task(
    task: &mut TaskContext,
    entry: extern "C" fn(usize, usize),
    arg1: usize,
    arg2: usize,
    stack_start: VirtAddr,
    is_user: bool,
) -> EResult<()> {
    internal::sched::init_task(task, entry, arg1, arg2, stack_start, is_user)
}

/// Transitions to user mode at a specified IP and SP.
/// # Safety
/// `ip` and `sp` have to point to valid and mapped addresses in the current address space.
pub unsafe fn jump_to_user(ip: VirtAddr, sp: VirtAddr) {
    unsafe { internal::sched::jump_to_user(ip, sp) };
}

/// Transitions to user mode with a specified context.
/// # Safety
/// `context` has to be allocated on the stack.
pub unsafe fn jump_to_user_context(context: *mut Context) {
    unsafe { internal::sched::jump_to_user_context(context) };
}
