//! Safe user memory reading/writing.

use super::VirtAddr;
use crate::generic::memory::virt::VmSpace;
use alloc::sync::Arc;
use bytemuck::AnyBitPattern;
use core::marker::PhantomData;

/// Provides safe access to memory from userland.
pub struct UserSlice<'a, T: AnyBitPattern> {
    map: Arc<VmSpace>,
    addr: VirtAddr,
    len: usize,
    _p: PhantomData<&'a T>,
}

impl<'a, T: AnyBitPattern> UserSlice<'a, T> {
    pub const fn new(map: Arc<VmSpace>, addr: VirtAddr, len: usize) -> Self {
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
