use super::spin::SpinLock;
use core::{
    cell::UnsafeCell,
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

/// A locking primitive for mutually exclusive accesses.
/// `T` is the type of the inner value to store.
pub struct Mutex<T: ?Sized> {
    inner: UnsafeCell<InnerMutex<T>>,
}

/// The inner workings of a [`Mutex`].
struct InnerMutex<T: ?Sized> {
    spin: SpinLock,
    data: T,
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            inner: UnsafeCell::new(InnerMutex {
                spin: SpinLock::new(),
                data,
            }),
        }
    }
}

impl<T: Default> Default for Mutex<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> MutexGuard<'_, T> {
        let inner = unsafe { &mut *self.inner.get() };
        inner.spin.lock();
        MutexGuard { parent: self }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self.is_locked() {
            return None;
        } else {
            return Some(self.lock());
        }
    }

    pub fn is_locked(&self) -> bool {
        let inner = unsafe { &*self.inner.get() };
        inner.spin.is_locked()
    }

    /// Forcefully unlocks this [`Mutex`].
    /// `irq` controls if IRQs should be reactivated after unlocking.
    /// # Safety
    /// The caller must ensure that enabling IRQs at this point is safe.
    pub unsafe fn force_unlock(&self) {
        let inner = unsafe { &mut (*self.inner.get()) };
        inner.spin.unlock();
    }
}

impl<T> Mutex<T> {
    /// Returns a pointer to the contained value.
    ///
    /// # Safety
    /// The caller must ensure that the contained data isn't accessed by a different caller.
    pub unsafe fn raw_inner(&self) -> *mut T {
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
unsafe impl<T> Sync for Mutex<T> {}

impl<T: ?Sized + Debug> Debug for Mutex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let guard = self.lock();
        Debug::fmt(&*guard, f)
    }
}

/// This struct is returned by [`Mutex::lock`] and is used to safely control mutex locking state.
pub struct MutexGuard<'m, T: ?Sized> {
    parent: &'m Mutex<T>,
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.parent.inner.get()).data }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.parent.inner.get()).data }
    }
}

/// A guard is only valid in the current thread and any attempt to move it out is illegal.
impl<T: ?Sized> !Send for MutexGuard<'_, T> {}

/// # Safety
/// We can guarantee that an acquired mutex context will never be accessed by two callers at the same time.
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

impl<T: ?Sized + Debug> Debug for MutexGuard<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            self.parent.force_unlock();
        }
    }
}
