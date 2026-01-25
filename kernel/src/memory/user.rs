//! Safe user memory reading/writing.

use super::VirtAddr;
use core::marker::PhantomData;

/// Provides safe access to a single structure from userland.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct UserPtr<T: Sized + Copy> {
    addr: VirtAddr,
    _p: PhantomData<T>,
}

// A UserPtr is always transparent and equal in size to a regular pointer.
static_assert!(size_of::<UserPtr<()>>() == size_of::<*const ()>());

impl<T: Sized + Copy> UserPtr<T> {
    pub const fn new(addr: VirtAddr) -> Self {
        Self {
            addr,
            _p: PhantomData,
        }
    }

    /// Creates a new pointer with an offset as a multiple of the underlying type.
    pub const fn offset(self, offset: usize) -> Self {
        Self {
            addr: VirtAddr::new(self.addr.0 + (offset * size_of::<T>())),
            _p: self._p,
        }
    }

    pub fn read(&self) -> Option<T> {
        // TODO: Mark the start of a user pointer access that can be checked in the PF handler.
        Some(unsafe { self.addr.as_ptr::<T>().read_unaligned() })
    }

    pub fn write(&mut self, value: T) -> bool {
        unsafe { self.addr.as_ptr::<T>().write_unaligned(value) };
        true
    }
}

impl<T: Sized + Copy> From<usize> for UserPtr<T> {
    fn from(value: usize) -> Self {
        Self::new(value.into())
    }
}

/// Provides safe access to a memory buffer from userland.
pub struct UserSlice<T: Sized + Copy> {
    addr: VirtAddr,
    len: usize,
    _p: PhantomData<T>,
}

impl<T: Sized + Copy> UserSlice<T> {
    pub const fn new(addr: VirtAddr, len: usize) -> Self {
        Self {
            addr,
            len,
            _p: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_slice(&self) -> Option<&[T]> {
        Some(unsafe { core::slice::from_raw_parts(self.addr.as_ptr::<T>(), self.len) })
    }

    pub fn as_mut_slice(&mut self) -> Option<&mut [T]> {
        Some(unsafe { core::slice::from_raw_parts_mut(self.addr.as_ptr::<T>(), self.len) })
    }
}
