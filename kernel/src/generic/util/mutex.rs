use super::spin::SpinLock;
use crate::arch;
use core::{
    cell::UnsafeCell,
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

/// An IRQ-safe mutex.
pub type IrqMutex<T> = Mutex<T, false>;
pub type IrqMutexGuard<'m, T> = MutexGuard<'m, T, false>;

/// A locking primitive for mutually exclusive accesses.
pub struct Mutex<T: ?Sized, const I: bool = true> {
    inner: UnsafeCell<InnerMutex<T, I>>,
}

impl<T, const I: bool> Mutex<T, I> {
    pub const fn new(data: T) -> Self {
        Self {
            inner: UnsafeCell::new(InnerMutex {
                spin: SpinLock::new(),
                data,
            }),
        }
    }
}

pub struct MutexGuard<'m, T: ?Sized, const INT: bool> {
    parent: &'m Mutex<T, INT>,
    irq: bool,
}

impl<T: ?Sized, const I: bool> Deref for MutexGuard<'_, T, I> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.parent.inner.get()).data }
    }
}

impl<T: ?Sized, const I: bool> DerefMut for MutexGuard<'_, T, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.parent.inner.get()).data }
    }
}

impl<T: ?Sized, const I: bool> !Send for MutexGuard<'_, T, I> {}

unsafe impl<T: ?Sized + Sync, const I: bool> Sync for MutexGuard<'_, T, I> {}

impl<T: ?Sized + Debug, const I: bool> Debug for MutexGuard<'_, T, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: ?Sized, const I: bool> Drop for MutexGuard<'_, T, I> {
    fn drop(&mut self) {
        unsafe {
            self.parent.force_unlock();
        }
    }
}

struct InnerMutex<T: ?Sized, const I: bool> {
    spin: SpinLock,
    data: T,
}

impl<T: Default, const I: bool> Default for Mutex<T, I> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: ?Sized, const I: bool> Mutex<T, I> {
    pub fn lock(&self) -> MutexGuard<T, I> {
        let irq = if !I {
            unsafe { arch::irq::set_irq_state(false) }
        } else {
            false
        };

        let inner = unsafe { &mut *self.inner.get() };
        inner.spin.lock();
        MutexGuard { parent: self, irq }
    }

    /// Forcefully unlocks this [`Mutex`].
    pub unsafe fn force_unlock(&self) {
        let inner = unsafe { &mut (*self.inner.get()) };
        inner.spin.unlock();
        if !I {
            unsafe { arch::irq::set_irq_state(true) };
        }
    }
}

impl<T, const I: bool> Mutex<T, I> {
    pub fn into_inner(self) -> T {
        let inner = unsafe { &mut *self.inner.get() };
        inner.spin.lock();
        self.inner.into_inner().data
    }
}

unsafe impl<T, const I: bool> Sync for Mutex<T, I> {}

impl<T: ?Sized + Debug, const I: bool> Debug for Mutex<T, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let guard = self.lock();
        Debug::fmt(&*guard, f)
    }
}
