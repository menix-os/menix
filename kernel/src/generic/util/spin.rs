use core::{
    hint,
    sync::atomic::{AtomicBool, Ordering},
};

/// A spin lock without a specific resource connected to it.
#[derive(Debug)]
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

    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}
