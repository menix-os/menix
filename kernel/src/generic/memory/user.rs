//! Safe user memory reading/writing.

use super::VirtAddr;
use crate::generic::memory::virt::AddressSpace;
use alloc::sync::Arc;
use bytemuck::AnyBitPattern;
use core::marker::PhantomData;

/// Provides safe access to a single structure from userland.
pub struct UserPtr<'a, T: AnyBitPattern> {
    addr: VirtAddr,
    _p: PhantomData<&'a T>,
}

impl<'a, T: AnyBitPattern> UserPtr<'a, T> {
    pub const fn new(addr: VirtAddr) -> Self {
        Self {
            addr,
            _p: PhantomData,
        }
    }

    pub fn read(&self) -> Option<T> {
        todo!()
    }

    pub fn write(&mut self) -> Option<T> {
        todo!()
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
