use crate::memory::UserPtr;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct iovec {
    pub base: UserPtr<()>,
    pub len: usize,
}
