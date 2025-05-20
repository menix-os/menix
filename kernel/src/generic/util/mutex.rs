use super::spin::SpinLock;
use crate::arch;
use core::{
    cell::UnsafeCell,
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

/// An IRQ-safe mutex.
pub type IrqMutex<T> = Mutex<T, true>;
pub type IrqMutexGuard<'m, T> = MutexGuard<'m, T, true>;

/// A locking primitive for mutually exclusive accesses.
/// `T` is the type of the inner value to store.
/// `I` indicates whether this [`Mutex`] is safe in IRQ-sensitive contexts.
pub struct Mutex<T: ?Sized, const I: bool = false> {
    inner: UnsafeCell<InnerMutex<T, I>>,
}

/// The inner workings of a [`Mutex`].
struct InnerMutex<T: ?Sized, const I: bool> {
    spin: SpinLock,
    data: T,
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

impl<T: Default, const I: bool> Default for Mutex<T, I> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: ?Sized, const I: bool> Mutex<T, I> {
    pub fn lock(&self) -> MutexGuard<T, I> {
        // Get the previous IRQ state.
        let irq = if I {
            // If we care about IRQ safety, disable IRQs at this point.
            unsafe { arch::irq::set_irq_state(false) }
        } else {
            // If we don't care about IRQ safety, just use false.
            false
        };

        let inner = unsafe { &mut *self.inner.get() };
        inner.spin.lock();
        MutexGuard { parent: self, irq }
    }

    pub fn is_locked(&self) -> bool {
        let inner = unsafe { &*self.inner.get() };
        inner.spin.is_locked()
    }

    /// Forcefully unlocks this [`Mutex`].
    /// `irq` controls if IRQs should be reactivated after unlocking.
    /// # Safety
    /// The caller must make sure that enabling IRQs at this point is safe.
    pub unsafe fn force_unlock(&self, irq: bool) {
        let inner = unsafe { &mut (*self.inner.get()) };
        inner.spin.unlock();

        // If we care about IRQ safety and the caller wants to, enable IRQs again.
        if I && irq {
            unsafe { arch::irq::set_irq_state(true) };
        }
    }
}

impl<T, const I: bool> Mutex<T, I> {
    pub unsafe fn inner(&self) -> &mut T {
        let inner = unsafe { &mut *self.inner.get() };
        &mut inner.data
    }

    pub fn into_inner(self) -> T {
        let inner = unsafe { &mut *self.inner.get() };
        inner.spin.lock();
        self.inner.into_inner().data
    }
}

/// # Safety
/// We can guarantee that types encapuslated by a [`Mutex`] are thread safe.
unsafe impl<T, const I: bool> Sync for Mutex<T, I> {}

impl<T: ?Sized + Debug, const I: bool> Debug for Mutex<T, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let guard = self.lock();
        Debug::fmt(&*guard, f)
    }
}

/// This struct is returned by [`Mutex::lock`] and is used to safely control mutex locking state.
pub struct MutexGuard<'m, T: ?Sized, const I: bool> {
    parent: &'m Mutex<T, I>,
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

/// A guard is only valid in the current thread and any attempt to move it out is illegal.
impl<T: ?Sized, const I: bool> !Send for MutexGuard<'_, T, I> {}

/// # Safety
/// We can guarantee that an acquired mutex context will never be accessed by two callers at the same time.
unsafe impl<T: ?Sized + Sync, const I: bool> Sync for MutexGuard<'_, T, I> {}

impl<T: ?Sized + Debug, const I: bool> Debug for MutexGuard<'_, T, I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: ?Sized, const I: bool> Drop for MutexGuard<'_, T, I> {
    fn drop(&mut self) {
        unsafe {
            self.parent.force_unlock(self.irq);
        }
    }
}
