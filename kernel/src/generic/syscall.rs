use crate::generic::log::GLOBAL_LOGGERS;
use core::{fmt::Write, ptr::slice_from_raw_parts};
use portal::syscall;

/// Executes the syscall as identified by `num`.
pub fn invoke(
    num: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> usize {
    let mut result = 0;

    match num {
        syscall::numbers::NULL => print!("NULL syscall invoked\n"),
        syscall::numbers::PRINT_LOG => {
            // let string = unsafe {
            //     let buffer = slice_from_raw_parts(a0 as *mut u8, a1).as_ref().unwrap();
            //     str::from_utf8(buffer)
            // };
            // GLOBAL_LOGGERS.lock().write_str(buffer).unwrap();
        }
        _ => print!("Unknown syscall requested by user program"),
    }

    return result;
}
