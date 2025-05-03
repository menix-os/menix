//! Safe user memory reading/writing.

use super::VirtAddr;
use core::{marker::PhantomData, ops::Deref};

/// Provides safe access to memory from userland.
pub struct UserBuffer<T> {
    addr: VirtAddr,
    len: usize,
    _p: PhantomData<T>,
}

impl<T> UserBuffer<T> {
    pub const fn new(addr: VirtAddr, len: usize) -> Self {
        Self {
            addr,
            len,
            _p: PhantomData,
        }
    }

    /// Gets the address of this buffer.
    pub const fn value(&self) -> VirtAddr {
        return self.addr;
    }

    /// Gets the length of this buffer.
    pub const fn len(&self) -> usize {
        return self.len;
    }
}
