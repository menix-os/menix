//! Helpers for structured data accesses.

use super::{PhysAddr, VirtAddr, pmm::KernelAlloc, virt::VmFlags};
use crate::generic::memory::virt::mmu::PageTable;
use core::marker::PhantomData;
use num_traits::{FromBytes, PrimInt, ToBytes};

pub trait MemoryView {
    /// Reads data from a register.
    fn read_reg<T: PrimInt + FromBytes>(&self, reg: Register<T>) -> Option<T>
    where
        T::Bytes: Default;

    /// Writes data to a register.
    fn write_reg<T: PrimInt + ToBytes>(&mut self, reg: Register<T>, value: T) -> Option<()>
    where
        T::Bytes: Default;

    /// Reads data from a field.
    fn read_field<T: PrimInt + From<A> + FromBytes, A: PrimInt + From<T> + FromBytes>(
        &self,
        field: Field<T, A>,
    ) -> Option<A>
    where
        T::Bytes: Default,
        A::Bytes: Default;

    /// Writes data to a field.
    fn write_field<T: PrimInt + FromBytes, A: PrimInt + FromBytes>(
        &self,
        field: Field<T, A>,
        value: A,
    ) -> Option<()>
    where
        T::Bytes: Default,
        A::Bytes: Default;
}

/// A hardware register mapped in the current address space.
/// All reads and writes must be properly aligned.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Register<T: PrimInt> {
    offset: usize,
    native_endian: bool,
    _p: PhantomData<T>,
}

impl<T: PrimInt> Register<T> {
    /// Creates a new register with native endianness.
    /// `offset` is in units of bytes.
    pub const fn new(offset: usize) -> Self {
        assert!(offset % size_of::<T>() == 0);
        Self {
            _p: PhantomData,
            offset,
            native_endian: true,
        }
    }

    /// Marks this field as little endian.
    pub const fn with_le(mut self) -> Self {
        self.native_endian = is_little_endian();
        self
    }

    /// Marks this field as little endian.
    pub const fn with_be(mut self) -> Self {
        self.native_endian = !is_little_endian();
        self
    }

    pub const fn offset(&self) -> usize {
        self.offset
    }
}

/// A [`Field`] is a subtype contained in a [`Register`].
/// `T` is a register type, and `A` is the type of the relevant part of that register.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Field<T: PrimInt, A: PrimInt> {
    field_offset: usize,
    register: Register<T>,
    _p: PhantomData<T>,
    _a: PhantomData<A>,
}

impl<T: PrimInt, A: PrimInt> Field<T, A> {
    /// Creates a new field with native endianness.
    pub const fn new(register: Register<T>, field_offset: usize) -> Self {
        assert!((field_offset + size_of::<A>()) <= size_of::<T>());
        Self {
            _p: PhantomData,
            _a: PhantomData,
            register,
            field_offset,
        }
    }

    pub const fn offset(&self) -> usize {
        self.register.offset() + self.field_offset
    }
}

const fn is_little_endian() -> bool {
    #[cfg(target_endian = "little")]
    return true;
    #[cfg(target_endian = "big")]
    return false;
}

impl MemoryView for [u8] {
    fn read_reg<T: PrimInt + FromBytes>(&self, reg: Register<T>) -> Option<T>
    where
        T::Bytes: Default,
    {
        let buf = self.get(reg.offset()..reg.offset() + size_of::<T>())?;
        let mut bytes = T::Bytes::default();
        bytes.as_mut().copy_from_slice(buf);
        let mut num = T::from_ne_bytes(&bytes);
        if !reg.native_endian {
            num = num.swap_bytes();
        }
        Some(num)
    }

    fn write_reg<T: PrimInt + ToBytes>(&mut self, reg: Register<T>, value: T) -> Option<()>
    where
        T::Bytes: Default,
    {
        let mut v = value;
        if !reg.native_endian {
            v = v.swap_bytes();
        }
        let bytes = T::to_ne_bytes(&v);
        let buf = self.get_mut(reg.offset()..reg.offset() + size_of::<T>())?;
        buf.copy_from_slice(bytes.as_ref());
        Some(())
    }

    fn read_field<T: PrimInt + From<A>, A: PrimInt + From<T>>(
        &self,
        field: Field<T, A>,
    ) -> Option<A> {
        todo!()
    }

    fn write_field<T: PrimInt, A: PrimInt>(&self, field: Field<T, A>, value: A) -> Option<()> {
        todo!()
    }
}

/// Represents a region of memory mapped IO.
#[derive(Debug)]
pub struct MmioView {
    /// A pointer to the start of this region in virtual memory.
    base: *mut (),
    /// The length of this region in bytes.
    len: usize,
}

unsafe impl Send for MmioView {}
unsafe impl Sync for MmioView {}

impl MmioView {
    /// Creates a new MMIO context over device memory.
    /// # Safety
    /// `phys` must be pointing to the start of the device memory region.
    pub unsafe fn new(phys: PhysAddr, len: usize) -> Self {
        return Self {
            // TODO: When adding memory type support, map this as uncacheable.
            base: PageTable::get_kernel()
                .map_memory::<KernelAlloc>(phys, VmFlags::Read | VmFlags::Write, len)
                .unwrap() as *mut (),
            len,
        };
    }
}

impl Drop for MmioView {
    fn drop(&mut self) {
        PageTable::get_kernel()
            .unmap_range::<KernelAlloc>(VirtAddr(self.base as usize), self.len)
            .unwrap();
    }
}

impl MemoryView for MmioView {
    fn read_reg<T: PrimInt>(&self, reg: Register<T>) -> Option<T> {
        if reg.offset() + size_of::<T>() > self.len {
            return None;
        }

        let value = unsafe { (self.base as *mut T).byte_add(reg.offset).read_volatile() };
        return Some(match reg.native_endian {
            true => value,
            false => value.swap_bytes(),
        });
    }

    fn write_reg<T: PrimInt>(&mut self, reg: Register<T>, value: T) -> Option<()> {
        if reg.offset() + size_of::<T>() > self.len {
            return None;
        }

        let value = match reg.native_endian {
            true => value,
            false => value.swap_bytes(),
        };
        unsafe {
            (self.base as *mut T)
                .byte_add(reg.offset)
                .write_volatile(value)
        };
        Some(())
    }

    fn read_field<T: PrimInt + From<A>, A: PrimInt + From<T>>(
        &self,
        field: Field<T, A>,
    ) -> Option<A> {
        todo!()
    }

    fn write_field<T: PrimInt, A: PrimInt>(&self, field: Field<T, A>, value: A) -> Option<()> {
        todo!()
    }
}
