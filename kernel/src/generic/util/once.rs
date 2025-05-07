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

    /// Initializes [`self`] with a concrete value.
    /// # Safety
    /// The caller must assert that this field has never been initialized before.
    pub unsafe fn init(&self, val: T) {
        unsafe { (*self.value.get()).write(val) };
    }

    /// Returns the inner value.
    #[inline]
    pub fn get(&self) -> &T {
        unsafe { (*self.value.get()).assume_init_ref() }
    }
}

unsafe impl<T> Sync for Once<T> {}
