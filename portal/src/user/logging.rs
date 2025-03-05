use crate::syscall::numbers;

pub fn log(message: &str) {
    super::do_syscall(
        numbers::PRINT_LOG,
        message.as_ptr() as usize,
        message.len(),
        0,
        0,
        0,
        0,
    );
}
