//! Helpers for structured MMIO accesses.

use super::{PhysAddr, VirtAddr, pmm::KernelAlloc, virt::VmFlags};
use crate::generic::memory::virt::mmu::PageTable;
use core::marker::PhantomData;
use num_traits::PrimInt;

/// Represents a region of memory mapped IO.
#[derive(Debug)]
pub struct Mmio {
    /// A pointer to the start of this region in virtual memory.
    base: *mut (),
    /// The length of this region in bytes.
    len: usize,
    /// Whether we made any virtual allocations we have to clean up.
    // TODO: Maybe a VMM can detect that?
    needs_unmap: bool,
}

unsafe impl Send for Mmio {}
unsafe impl Sync for Mmio {}

impl Mmio {
    /// Creates a new MMIO context over device memory.
    /// # Safety
    /// `phys` must be pointing to the start of the device memory region.
    pub unsafe fn new_mmio(phys: PhysAddr, len: usize) -> Self {
        return Self {
            // TODO: When adding memory type support, map this as uncacheable.
            base: PageTable::get_kernel()
                .map_memory::<KernelAlloc>(phys, VmFlags::Read | VmFlags::Write, len)
                .unwrap() as *mut (),
            needs_unmap: true,
            len,
        };
    }

    /// Creates a new MMIO context over allocated memory.
    /// # Safety
    /// `addr` must be a valid address within the kernel page table.
    pub unsafe fn new_memory(addr: *mut (), len: usize) -> Self {
        return Self {
            base: addr,
            needs_unmap: false,
            len,
        };
    }

    /// The length of this memory space.
    pub const fn len(&self) -> usize {
        return self.len;
    }

    /// Reads data from a register.
    pub fn read<T: PrimInt>(&self, reg: Register<T>) -> T {
        let value = unsafe { (self.base as *mut T).byte_add(reg.offset).read_volatile() };
        return match reg.native_endian {
            true => value,
            false => value.swap_bytes(),
        };
    }

    /// Writes data to a register.
    pub fn write<T: PrimInt>(&self, reg: Register<T>, value: T) {
        let value = match reg.native_endian {
            true => value,
            false => value.swap_bytes(),
        };
        unsafe {
            (self.base as *mut T)
                .byte_add(reg.offset)
                .write_volatile(value)
        };
    }

    /// Reads data from a field.
    pub fn read_field<T: PrimInt + From<A>, A: PrimInt + From<T>>(&self, field: Field<T, A>) -> A {
        todo!()
    }

    /// Writes data to a field.
    pub fn write_field<T: PrimInt, A: PrimInt>(&self, field: Field<T, A>, value: A) {
        todo!()
    }
}

impl Drop for Mmio {
    fn drop(&mut self) {
        if self.needs_unmap {
            PageTable::get_kernel()
                .unmap_range::<KernelAlloc>(VirtAddr(self.base as usize), self.len)
                .unwrap();
        }
    }
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
