pub mod irq;
pub mod spin;

use crate::generic::{
    percpu::CpuData, process::task::Task, sched::Scheduler, util::mutex::spin::SpinMutex,
};
use alloc::sync::Arc;
use core::{
    cell::UnsafeCell,
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};
use intrusive_collections::{LinkedList, LinkedListAtomicLink, UnsafeRef, intrusive_adapter};

intrusive_adapter!(WaitersLinkAdapter = UnsafeRef<Waiter>: Waiter { waiters_link: LinkedListAtomicLink });

struct Waiter {
    waiters_link: LinkedListAtomicLink,
    task: Arc<Task>,
}

struct MutexInner {
    owner: Option<Arc<Task>>,
    waiters: LinkedList<WaitersLinkAdapter>,
}

pub struct Mutex<T: ?Sized> {
    flag: AtomicBool,
    inner: SpinMutex<MutexInner>,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'m, T: ?Sized> {
    mutex: &'m Mutex<T>,
}

impl<T: ?Sized> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T: ?Sized> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T: ?Sized> !Send for MutexGuard<'_, T> {}

unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}

impl<T: ?Sized + Debug> Debug for MutexGuard<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            flag: AtomicBool::new(false),
            inner: SpinMutex::new(MutexInner {
                owner: None,
                waiters: LinkedList::new(WaitersLinkAdapter::NEW),
            }),
            data: UnsafeCell::new(data),
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> MutexGuard<'_, T> {
        if self
            .flag
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // let task = Scheduler::get_current();
            // status!("waiting for mutex task {}", task.get_id());
            let waiter = Waiter {
                waiters_link: LinkedListAtomicLink::new(),
                task: Scheduler::get_current(),
            };

            self.inner
                .lock()
                .waiters
                .push_back(unsafe { UnsafeRef::from_raw(&waiter) });

            CpuData::get().scheduler.do_yield();
            // status!("i woke up yay");
        }

        let mut inner = self.inner.lock();
        debug_assert!(inner.owner.is_none());
        inner.owner = Some(Scheduler::get_current());
        MutexGuard { mutex: self }
    }

    pub fn unlock(&self) {
        let mut inner = self.inner.lock();
        debug_assert!(
            inner
                .owner
                .as_ref()
                .map(|owner| Arc::ptr_eq(owner, &Scheduler::get_current()))
                .unwrap_or(false)
        );

        inner.owner = None;

        if let Some(waiter) = inner.waiters.pop_front() {
            // status!("waking up task {}", waiter.task.get_id());
            CpuData::get().scheduler.add_task(waiter.task.clone());
        } else {
            self.flag.store(false, Ordering::Release);
        }
    }
}
