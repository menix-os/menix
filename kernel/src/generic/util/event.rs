use crate::generic::{
    percpu::CpuData, process::task::Task, sched::Scheduler, util::mutex::spin::SpinMutex,
};
use alloc::{boxed::Box, sync::Arc};
use intrusive_collections::{LinkedList, LinkedListAtomicLink, intrusive_adapter};

#[derive(Debug)]
struct Waiter {
    waiters_link: LinkedListAtomicLink,
    task: Arc<Task>,
}

intrusive_adapter!(WaitersLinkAdapter = Box<Waiter>: Waiter { waiters_link: LinkedListAtomicLink });

pub struct Event {
    waiters: SpinMutex<LinkedList<WaitersLinkAdapter>>,
}

impl Event {
    pub fn new() -> Self {
        Self {
            waiters: SpinMutex::new(LinkedList::new(WaitersLinkAdapter::NEW)),
        }
    }

    pub fn guard(&self) -> EventGuard<'_> {
        let mut waiters = self.waiters.lock();
        waiters.push_back(Box::new(Waiter {
            waiters_link: LinkedListAtomicLink::new(),
            task: Scheduler::get_current(),
        }));

        return EventGuard { parent: self };
    }

    pub fn wake_one(&self) {
        let mut waiters = self.waiters.lock();
        if let Some(waiter) = waiters.pop_front() {
            CpuData::get().scheduler.add_task(waiter.task.clone());
        }
    }

    pub fn wake_all(&self) {
        let mut waiters = self.waiters.lock();
        for waiter in waiters.iter() {
            CpuData::get().scheduler.add_task(waiter.task.clone());
        }
        waiters.clear();
    }
}

pub struct EventGuard<'n> {
    parent: &'n Event,
}

impl<'n> EventGuard<'n> {
    pub fn wait(&self) {
        CpuData::get().scheduler.do_yield();
    }
}
