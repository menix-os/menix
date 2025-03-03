// Logging facilities.

use crate::internal;

pub fn log(message: &str) {
    internal::do_syscall(0, message.as_ptr() as usize, message.len(), 0, 0, 0, 0);
}
