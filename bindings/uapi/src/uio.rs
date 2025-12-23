#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct iovec {
    pub base: *mut (),
    pub len: usize,
}
