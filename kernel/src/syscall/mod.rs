// System call interface

/// Checks if the syscall is present and calls the kernel function with sanitized input.
/// This function uses the menix syscall ABI.
#[cfg(not(feature = "linux_abi"))]
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

/// Checks if the syscall is present and calls the kernel function with sanitized input.
/// This function uses the Linux syscall ABI.
#[cfg(feature = "linux_abi")]
pub fn invoke(
    num: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> usize {
    todo!()
}
