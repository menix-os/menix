//! Helpers for structured MMIO accesses.

use super::{
    PhysAddr, VirtAddr,
    pmm::FreeList,
    virt::{self, VmFlags, VmLevel},
};
use core::marker::PhantomData;
use num_traits::PrimInt;

/// Represents a region of memory mapped IO.
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
            base: virt::KERNEL_PAGE_TABLE
                .lock()
                .map_memory::<FreeList>(phys, VmFlags::Read | VmFlags::Write, VmLevel::L1, len)
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

    /// Reads data from a single field.
    pub fn read<T: PrimInt>(&self, field: &MmioField<T>) -> T {
        let value = unsafe { (self.base as *mut T).byte_add(field.offset).read_volatile() };
        return match field.native_endian {
            true => value,
            false => value.swap_bytes(),
        };
    }

    /// Writes data to a single field.
    pub fn write<T: PrimInt>(&mut self, field: &MmioField<T>, value: T) {
        unsafe {
            (self.base as *mut T).byte_add(field.offset).write_volatile(
                match field.native_endian {
                    true => value,
                    false => value.swap_bytes(),
                },
            );
        }
    }

    /// Reads multiple elements into a buffer.
    pub fn read_array<T: PrimInt>(&self, vector: &MmioArray<T>, offset: usize, dest: &mut [T]) {
        assert!(dest.len() == vector.count);
        for (idx, elem) in dest.iter_mut().enumerate() {
            *elem = self.read_at(vector, offset + idx);
        }
    }

    /// Writes multiple array elements from a buffer.
    pub fn write_array<T: PrimInt>(&mut self, vector: &MmioArray<T>, offset: usize, value: &[T]) {
        assert!(value.len() == vector.count);
        for (idx, elem) in value.iter().enumerate() {
            self.write_at(vector, offset + idx, *elem);
        }
    }

    /// Reads a single element from a vector.
    pub fn read_at<T: PrimInt>(&self, vector: &MmioArray<T>, index: usize) -> T {
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
    pub fn write_at<T: PrimInt>(&mut self, vector: &MmioArray<T>, index: usize, value: T) {
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
}

impl Drop for Mmio {
    fn drop(&mut self) {
        if self.needs_unmap {
            virt::KERNEL_PAGE_TABLE
                .lock()
                .unmap_range(VirtAddr(self.base as usize), self.len)
                .unwrap();
        }
    }
}

/// Single member of a structure.
#[derive(Debug)]
pub struct MmioField<T: PrimInt> {
    _p: PhantomData<T>,
    offset: usize,
    native_endian: bool,
}

impl<T: PrimInt> MmioField<T> {
    /// Creates a new field with native endianness.
    /// `offset` is in units of bytes.
    pub const fn new(offset: usize) -> Self {
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
}

pub struct MmioArray<T> {
    _p: PhantomData<T>,
    offset: usize,
    stride: usize,
    count: usize,
    native_endian: bool,
}

impl<T> MmioArray<T> {
    pub const fn new(offset: usize, count: usize) -> Self {
        Self {
            _p: PhantomData,
            offset,
            stride: size_of::<T>(),
            count,
            native_endian: true,
        }
    }

    /// Marks this array as little endian.
    pub const fn with_le(mut self) -> Self {
        self.native_endian = is_little_endian();
        self
    }

    /// Marks this array as little endian.
    pub const fn with_be(mut self) -> Self {
        self.native_endian = !is_little_endian();
        self
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
