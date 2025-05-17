use super::internal;
use crate::generic::errno::Errno;
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
/// # Note
/// The implementation of this function must be an atomic operation for this to be memory safe!
pub fn get_task() -> *mut Task {
    internal::sched::get_task()
}

/// Switches the current CPU context from one task to another.
pub unsafe fn switch(from: *mut Task, to: *mut Task) -> *mut Task {
    unsafe { internal::sched::switch(from, to) }
}

pub fn init_task(
    task: &mut TaskContext,
    entry: extern "C" fn(usize) -> !,
    arg: usize,
    stack_start: usize,
    is_user: bool,
) -> Result<(), Errno> {
    internal::sched::init_task(task, entry, arg, stack_start, is_user)
}

/// Transitions to user mode at a specified IP and SP.
/// # Safety
/// `ip` and `sp` have to point to valid and mapped addresses in the current address space.
pub unsafe fn jump_to_user(ip: usize, sp: usize) {
    unsafe { internal::sched::jump_to_user(ip, sp) };
}

/// Transitions to user mode with a specified context.
/// # Safety
/// `context` has to be allocated on the stack.
pub unsafe fn jump_to_user_context(context: *mut Context) {
    unsafe { internal::sched::jump_to_user_context(context) };
}
