use crate::{syscall::numbers::THREAD_EXIT, user::do_syscall};

pub fn exit() -> ! {
    do_syscall(THREAD_EXIT, 0, 0, 0, 0, 0, 0);
    unreachable!();
}
