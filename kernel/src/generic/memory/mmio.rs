// Helpers for specification compliant structure definitions.

use num_traits::PrimInt;

use super::virt::{self, VmFlags};
use crate::arch::{PhysAddr, VirtAddr};
use core::{marker::PhantomData, ops::RangeInclusive};

pub struct MemorySpace {
    phys: PhysAddr,
    base: *mut u8,
    len: usize,
}

unsafe impl Send for MemorySpace {}
unsafe impl Sync for MemorySpace {}

impl Drop for MemorySpace {
    fn drop(&mut self) {
        virt::KERNEL_PAGE_TABLE
            .write()
            .unmap_range(self.base as VirtAddr, self.len);
    }
}

impl MemorySpace {
    pub fn new(phys: PhysAddr, len: usize) -> Self {
        return Self {
            phys,
            base: virt::KERNEL_PAGE_TABLE.write().map_memory(
                phys,
                VmFlags::Read | VmFlags::Write,
                0,
                len,
            ),
            len,
        };
    }

    /// The length of this memory space.
    pub const fn len(&self) -> usize {
        return self.len;
    }

    /// Reads data from a single field.
    pub fn read_field<T: PrimInt>(&self, field: &MemoryField<T>) -> T {
        let result = T::zero();
        let value = unsafe { (self.base as *mut T).add(field.offset).read_volatile() };
        return match field.native_endian {
            true => value,
            false => value.swap_bytes(),
        };
    }

    /// Writes data to a single field.
    pub fn write_field<T: PrimInt>(&mut self, field: &MemoryField<T>, value: T) {
        unsafe {
            (self.base as *mut T).byte_add(field.offset).write_volatile(
                match field.native_endian {
                    true => value,
                    false => value.swap_bytes(),
                },
            );
        }
    }

    /// Reads a single element from a vector.
    pub fn vector_read_elem<T: PrimInt>(&self, vector: &MemoryVector<T>, index: usize) -> T {
        assert!(index < vector.count);
        let value = unsafe {
            (self.base as *const T)
                .byte_add(vector.offset + (vector.stride * index))
                .read_volatile()
        };
        return match vector.native_endian {
            true => value,
            false => value.swap_bytes(),
        };
    }

    /// Writes a single element to a vector.
    pub fn vector_write_elem<T: PrimInt>(
        &mut self,
        vector: &MemoryVector<T>,
        index: usize,
        value: T,
    ) {
        assert!(index < vector.count);
        unsafe {
            (self.base as *mut T)
                .byte_add(vector.offset + (vector.stride * index))
                .write_volatile(match vector.native_endian {
                    true => value,
                    false => value.swap_bytes(),
                });
        }
    }

    /// Reads multiple vector elements into a buffer.
    pub fn vector_read<T: PrimInt>(&self, vector: &MemoryVector<T>, offset: usize, dest: &mut [T]) {
        for (idx, elem) in dest.iter_mut().enumerate() {
            *elem = self.vector_read_elem(vector, offset + idx);
        }
    }

    pub fn vector_write<T: PrimInt>(
        &mut self,
        vector: &MemoryVector<T>,
        offset: usize,
        value: &[T],
    ) {
        for (idx, elem) in value.iter().enumerate() {
            self.vector_write_elem(vector, offset + idx, *elem);
        }
    }
}

/// Single member of a structure.
#[derive(Debug)]
pub struct MemoryField<T: PrimInt> {
    _p: PhantomData<T>,
    offset: usize,
    len: usize,
    native_endian: bool,
}

impl<T: PrimInt> MemoryField<T> {
    /// Creates a new field with native endianness.
    /// `offset` is in units of bytes.
    pub const fn new_ne(offset: usize) -> Self {
        Self {
            _p: PhantomData,
            offset,
            len: size_of::<T>(),
            native_endian: true,
        }
    }

    /// Creates a new field with little endianness.
    /// `offset` is in units of bytes.
    pub const fn new_le(offset: usize) -> Self {
        Self {
            _p: PhantomData,
            offset,
            len: size_of::<T>(),
            native_endian: is_little_endian(),
        }
    }

    /// Creates a new field with big endianness.
    /// `offset` is in units of bytes.
    pub const fn new_be(offset: usize) -> Self {
        Self {
            _p: PhantomData,
            offset,
            len: size_of::<T>(),
            native_endian: !is_little_endian(),
        }
    }

    /// Creates a new field with native endianness.
    /// `offset` and `len` are in units of bytes.
    pub const fn new_ne_with_len(offset: usize, len: usize) -> Self {
        Self {
            _p: PhantomData,
            offset,
            len,
            native_endian: true,
        }
    }

    /// Creates a new field with little endianness.
    /// `offset` and `len` are in units of bytes.
    pub const fn new_le_with_len(offset: usize, len: usize) -> Self {
        Self {
            _p: PhantomData,
            offset,
            len,
            native_endian: is_little_endian(),
        }
    }

    /// Creates a new field with big endianness.
    /// `offset` and `len` are in units of bytes.
    pub const fn new_be_with_len(offset: usize, len: usize) -> Self {
        Self {
            _p: PhantomData,
            offset,
            len,
            native_endian: !is_little_endian(),
        }
    }
}

/// A bit field.
pub struct MemoryBitField {
    start: usize,
    len: usize,
}

impl MemoryBitField {
    /// Creates a new bit field starting at bit `start`, with a length of `len` bits.
    pub const fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    /// Creates a new bit field starting at bit `start` and ending at bit `end`.
    /// `end` is inclusive.
    pub const fn new_range(range: RangeInclusive<usize>) -> Self {
        Self {
            start: *range.start(),
            len: *range.end() - *range.start() + 1,
        }
    }
}

pub struct MemoryVector<T> {
    _p: PhantomData<T>,
    offset: usize,
    stride: usize,
    count: usize,
    native_endian: bool,
}

impl<T> MemoryVector<T> {
    pub const fn new_ne(offset: usize, count: usize) -> Self {
        Self {
            _p: PhantomData,
            offset,
            stride: size_of::<T>(),
            count,
            native_endian: true,
        }
    }

    pub const fn new_ne_with_stride(offset: usize, count: usize, stride: usize) -> Self {
        assert!(stride >= size_of::<T>(), "Elements may not overlap");

        Self {
            _p: PhantomData,
            offset,
            stride,
            count,
            native_endian: true,
        }
    }

    pub const fn new_le(offset: usize, count: usize) -> Self {
        Self {
            _p: PhantomData,
            offset,
            stride: size_of::<T>(),
            count,
            native_endian: is_little_endian(),
        }
    }

    pub const fn new_le_with_stride(offset: usize, count: usize, stride: usize) -> Self {
        assert!(stride >= size_of::<T>(), "Elements may not overlap");

        Self {
            _p: PhantomData,
            offset,
            stride,
            count,
            native_endian: is_little_endian(),
        }
    }

    pub const fn new_be(offset: usize, count: usize) -> Self {
        Self {
            _p: PhantomData,
            offset,
            stride: size_of::<T>(),
            count,
            native_endian: !is_little_endian(),
        }
    }

    pub const fn new_be_with_stride(offset: usize, count: usize, stride: usize) -> Self {
        assert!(stride >= size_of::<T>(), "Elements may not overlap");

        Self {
            _p: PhantomData,
            offset,
            stride,
            count,
            native_endian: !is_little_endian(),
        }
    }
}

const fn is_little_endian() -> bool {
    #[cfg(target_endian = "little")]
    return true;
    #[cfg(target_endian = "big")]
    return false;
}
