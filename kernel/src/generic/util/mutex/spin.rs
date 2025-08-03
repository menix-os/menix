use core::{
    cell::UnsafeCell,
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

use crate::generic::util::spin::SpinLock;

/// A locking primitive for mutually exclusive accesses.
/// `T` is the type of the inner value to store.
pub struct SpinMutex<T: ?Sized> {
    inner: UnsafeCell<InnerSpinMutex<T>>,
}

/// The inner workings of a [`SpinMutex`].
struct InnerSpinMutex<T: ?Sized> {
    spin: SpinLock,
    data: T,
}

impl<T> SpinMutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            inner: UnsafeCell::new(InnerSpinMutex {
                spin: SpinLock::new(),
                data,
            }),
        }
    }
}

impl<T: Default> Default for SpinMutex<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: ?Sized> SpinMutex<T> {
    pub fn lock(&self) -> SpinMutexGuard<'_, T> {
        let inner = unsafe { &mut *self.inner.get() };
        inner.spin.lock();
        SpinMutexGuard { parent: self }
    }

    pub fn try_lock(&self) -> Option<SpinMutexGuard<'_, T>> {
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

    /// Forcefully unlocks this [`SpinMutex`].
    /// # Safety
    /// The caller must ensure that unlocking the mutex at this point is safe.
    pub unsafe fn force_unlock(&self) {
        let inner = unsafe { &mut (*self.inner.get()) };
        inner.spin.unlock();
    }
}

impl<T> SpinMutex<T> {
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
unsafe impl<T> Sync for SpinMutex<T> {}

impl<T: ?Sized + Debug> Debug for SpinMutex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let guard = self.lock();
        Debug::fmt(&*guard, f)
    }
}

/// This struct is returned by [`Mutex::lock`] and is used to safely control mutex locking state.
pub struct SpinMutexGuard<'m, T: ?Sized> {
    parent: &'m SpinMutex<T>,
}

impl<T: ?Sized> Deref for SpinMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.parent.inner.get()).data }
    }
}

impl<T: ?Sized> DerefMut for SpinMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.parent.inner.get()).data }
    }
}

/// A guard is only valid in the current thread and any attempt to move it out is illegal.
impl<T: ?Sized> !Send for SpinMutexGuard<'_, T> {}

/// # Safety
/// We can guarantee that an acquired mutex context will never be accessed by two callers at the same time.
unsafe impl<T: ?Sized + Sync> Sync for SpinMutexGuard<'_, T> {}

impl<T: ?Sized + Debug> Debug for SpinMutexGuard<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: ?Sized> Drop for SpinMutexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            self.parent.force_unlock();
        }
    }
}
