//! Safe user memory reading/writing.

use super::VirtAddr;
use crate::generic::sched::Scheduler;
use bytemuck::AnyBitPattern;
use core::marker::PhantomData;

/// Provides safe access to a single structure from userland.
pub struct UserPtr<'a, T: Sized + Copy> {
    addr: VirtAddr,
    _p: PhantomData<&'a T>,
}

impl<'a, T: Sized + Copy> UserPtr<'a, T> {
    pub const fn new(addr: VirtAddr) -> Self {
        Self {
            addr,
            _p: PhantomData,
        }
    }

    pub fn read(&self) -> Option<T> {
        let current = Scheduler::get_current().get_process();
        let space = &current.inner.lock().address_space;
        if !space.is_mapped(self.addr, size_of::<T>()) {
            None
        } else {
            Some(unsafe { self.addr.as_ptr::<T>().read_unaligned() })
        }
    }

    pub fn write(&self, value: T) -> bool {
        let current = Scheduler::get_current().get_process();
        let space = &current.inner.lock().address_space;
        if !space.is_mapped(self.addr, size_of::<T>()) {
            false
        } else {
            unsafe { self.addr.as_ptr::<T>().write_unaligned(value) };
            true
        }
    }
}

impl<'a, T: Sized + Copy> From<usize> for UserPtr<'a, T> {
    fn from(value: usize) -> Self {
        Self::new(value.into())
    }
}

/// Provides safe access to a memory buffer from userland.
pub struct UserSlice<'a, T: AnyBitPattern> {
    addr: VirtAddr,
    len: usize,
    _p: PhantomData<&'a T>,
}

impl<'a, T: AnyBitPattern> UserSlice<'a, T> {
    pub const fn new(addr: VirtAddr, len: usize) -> Self {
        Self {
            addr,
            len,
            _p: PhantomData,
        }
    }

    pub fn as_slice(&self) -> Option<&'a [T]> {
        let current = Scheduler::get_current().get_process();
        let space = &current.inner.lock().address_space;
        if !space.is_mapped(self.addr, size_of::<T>() * self.len) {
            None
        } else {
            Some(unsafe { core::slice::from_raw_parts(self.addr.as_ptr::<T>(), self.len) })
        }
    }

    pub fn as_mut_slice(&mut self) -> Option<&'a mut [T]> {
        let current = Scheduler::get_current().get_process();
        let space = &current.inner.lock().address_space;
        if !space.is_mapped(self.addr, size_of::<T>() * self.len) {
            None
        } else {
            Some(unsafe { core::slice::from_raw_parts_mut(self.addr.as_ptr::<T>(), self.len) })
        }
    }
}
