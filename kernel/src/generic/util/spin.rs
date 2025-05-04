use core::{
    hint,
    sync::atomic::{AtomicBool, Ordering},
};

/// A spin lock without a specific resource connected to it.
pub struct SpinLock(AtomicBool);

impl SpinLock {
    pub const fn new() -> Self {
        Self(AtomicBool::new(false))
    }

    #[inline(always)]
    pub fn lock(&mut self) {
        while self.0.swap(true, Ordering::Acquire) {
            hint::spin_loop();
        }
    }

    #[inline(always)]
    pub fn unlock(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}
