//! Helpers for structured data accesses.

use super::{PhysAddr, VirtAddr, pmm::KernelAlloc, virt::VmFlags};
use crate::generic::memory::virt::mmu::PageTable;
use core::{marker::PhantomData, ops::RangeInclusive};
use num_traits::{FromBytes, PrimInt, ToBytes};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitValue<T: PrimInt> {
    value: T,
}

fn field_mask<T: PrimInt, A: PrimInt + Into<T>>(field: Field<T, A>) -> T {
    let mask: T = A::one().into();
    (mask << field.bit_width) - A::one().into()
}

impl<T: PrimInt> BitValue<T> {
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    pub const fn value(&self) -> T {
        self.value
    }

    pub fn read_field<A: PrimInt + TryFrom<T> + Into<T>>(self, field: Field<T, A>) -> BitValue<A> {
        let value = (self.value >> field.field_offset) & field_mask(field);
        BitValue::new(value.try_into().ok().unwrap())
    }

    pub fn write_field<A: PrimInt + From<T>>(self, field: Field<T, A>, value: A) -> Self
    where
        T: From<A>,
    {
        let value: T = value.into();
        BitValue::new(
            (self.value & !(field_mask(field) << field.field_offset))
                | (value << field.field_offset),
        )
    }
}

pub trait MemoryView {
    /// Reads data from a register.
    fn read_reg<T: PrimInt + FromBytes>(&self, reg: Register<T>) -> Option<BitValue<T>>
    where
        T::Bytes: Default;

    /// Writes data to a register.
    fn write_reg<T: PrimInt + ToBytes>(&mut self, reg: Register<T>, value: T) -> Option<()>
    where
        T::Bytes: Default;
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
    bit_width: usize,
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
            field_offset: field_offset * 8,
            bit_width: size_of::<A>() * 8,
        }
    }

    /// Creates a new field spanning the given bit range (inclusive).
    pub const fn new_bit(register: Register<T>, range: RangeInclusive<usize>) -> Self {
        let start = *range.start();
        let end = *range.end();
        assert!(start <= end);
        assert!(end < size_of::<T>() * u8::BITS as usize);
        let width = end - start + 1;
        assert!(width <= size_of::<A>() * u8::BITS as usize);
        Self {
            _a: PhantomData,
            _p: PhantomData,
            register,
            field_offset: start,
            bit_width: width,
        }
    }

    pub const fn byte_offset(&self) -> usize {
        self.register.offset() + (self.field_offset / 8)
    }

    pub const fn bit_offset(&self) -> usize {
        self.field_offset
    }

    pub const fn bit_width(&self) -> usize {
        self.bit_width
    }
}

const fn is_little_endian() -> bool {
    #[cfg(target_endian = "little")]
    return true;
    #[cfg(target_endian = "big")]
    return false;
}

impl MemoryView for [u8] {
    fn read_reg<T: PrimInt + FromBytes>(&self, reg: Register<T>) -> Option<BitValue<T>>
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
        Some(BitValue::new(num))
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
    fn read_reg<T: PrimInt>(&self, reg: Register<T>) -> Option<BitValue<T>> {
        if reg.offset() + size_of::<T>() > self.len {
            return None;
        }

        let mut value = unsafe { (self.base as *mut T).byte_add(reg.offset).read_volatile() };
        if !reg.native_endian {
            value = value.swap_bytes();
        }

        Some(BitValue::new(value))
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
}
