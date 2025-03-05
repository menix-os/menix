use crate::syscall;

pub fn allocate(size: usize, alignment: usize) -> *mut u8 {
    let (result, error) = super::do_syscall(
        syscall::numbers::MEMORY_ALLOCATE,
        size,
        alignment,
        0,
        0,
        0,
        0,
    );
    assert_eq!(
        error, 0,
        "Unable to allocate memory with error code {}",
        error
    );
    return result as *mut u8;
}

pub fn free(ptr: *mut u8, size: usize, alignment: usize) {
    super::do_syscall(
        syscall::numbers::MEMORY_FREE,
        ptr as usize,
        size,
        alignment,
        0,
        0,
        0,
    );
}
