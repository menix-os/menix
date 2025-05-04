use core::{cell::UnsafeCell, mem::MaybeUninit};

#[repr(transparent)]
pub struct Once<T> {
    value: UnsafeCell<MaybeUninit<T>>,
}

impl<T> Once<T> {
    pub const fn new() -> Self {
        Self {
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub unsafe fn init(&self, val: T) {
        unsafe { (*self.value.get()).write(val) };
    }

    /// Returns the inner value.
    pub fn get(&self) -> &T {
        unsafe { (*self.value.get()).assume_init_ref() }
    }
}

unsafe impl<T> Sync for Once<T> {}
