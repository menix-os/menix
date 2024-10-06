use core::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Relaxed, Release},
};

use crate::arch::*;

/// A spinlock stops execution
#[derive(Debug)]
pub struct SpinLock {
    /// The CPU ID connected to the owner.
    cpu: usize,
    /// Whether it's locked or not.
    locked: AtomicBool,
}

// Disables or enables the use of spinlocks. They have to be disabled during platform init.
static SPINLOCKS_ACTIVE: AtomicBool = AtomicBool::new(false);

impl Default for SpinLock {
    fn default() -> Self {
        Self::new()
    }
}

impl SpinLock {
    /// Creates a new, unlocked spinlock.
    pub const fn new() -> Self {
        Self {
            cpu: 0,
            locked: AtomicBool::new(false),
        }
    }

    /// Activates all spinlocks.
    pub fn activate() {
        SPINLOCKS_ACTIVE.store(true, Release);
    }

    /// Deactivates all spinlocks.
    pub fn deactivate() {
        SPINLOCKS_ACTIVE.store(false, Release);
    }

    /// Attempts to acquire this spinlock once.
    /// Returns true if successful.
    pub fn acquire(&mut self) -> bool {
        // If spinlocks are disabled, all accesses are succesful.
        if !SPINLOCKS_ACTIVE.load(Acquire) {
            return true;
        }

        // If the current value of `locked` is `false`, replace it with `true`.
        match self.locked.compare_exchange(false, true, Acquire, Relaxed) {
            // We're the first to acquire this lock. Mark it as locked and record our CPU ID.
            Ok(_) => {
                self.locked.store(true, Release);
                self.cpu = Arch::current_cpu().id();
                true
            }
            // Lock already has been acquired by someone else.
            Err(_) => false,
        }
    }

    pub fn acquire_force(&mut self) -> Self {
        todo!();
    }

    /// Frees the lock if it was previously locked.
    pub fn free(&mut self) {}
}

impl Drop for SpinLock {
    fn drop(&mut self) {
        self.free();
    }
}
