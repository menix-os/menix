// System call interface

/// Checks if the syscall is present and calls the kernel function with sanitized input.
/// This assumes menix syscall ABI.
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
        _ => todo!("Unhandled syscall"),
    }

    return result;
}
