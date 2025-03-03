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
        _ => print!("Unknown syscall requested by user program"),
    }

    return result;
}
