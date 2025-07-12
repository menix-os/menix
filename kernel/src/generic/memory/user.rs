//! Safe user memory reading/writing.

use super::VirtAddr;
use crate::generic::memory::virt::{AddressSpace, PageTable};
use alloc::sync::Arc;
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
        if !PageTable::get_kernel().is_mapped(self.addr) {
            return None;
        }

        return Some(unsafe { self.addr.as_ptr::<T>().read_unaligned() });
    }

    pub fn write(&self, value: T) {
        if !PageTable::get_kernel().is_mapped(self.addr) {
            return;
        }

        unsafe { self.addr.as_ptr::<T>().write_unaligned(value) };
    }
}

impl<'a, T: Sized + Copy> From<usize> for UserPtr<'a, T> {
    fn from(value: usize) -> Self {
        Self::new(value.into())
    }
}

/// Provides safe access to a memory buffer from userland.
pub struct UserSlice<'a, T: AnyBitPattern> {
    map: Arc<AddressSpace>,
    addr: VirtAddr,
    len: usize,
    _p: PhantomData<&'a T>,
}

impl<'a, T: AnyBitPattern> UserSlice<'a, T> {
    pub const fn new(map: Arc<AddressSpace>, addr: VirtAddr, len: usize) -> Self {
        Self {
            map,
            addr,
            len,
            _p: PhantomData,
        }
    }

    pub fn as_slice(&self) -> &'a [T] {
        todo!()
    }

    pub fn as_mut_slice(&mut self) -> &'a mut [T] {
        todo!()
    }
}
