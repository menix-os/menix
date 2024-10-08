// Bit map storage

use crate::thread::spin::SpinLock;
use core::ptr::null_mut;

#[derive(Debug)]
pub struct BitMap {
    /// Access lock.
    lock: SpinLock,
    /// Storage backing this bit map.
    data: *mut u8,
    /// Length of the bit map buffer in bytes.
    size: usize,
}

// It's safe, trust me bro.
unsafe impl Sync for BitMap {}

impl BitMap {
    /// Creates a new empty bit map.
    pub const fn new() -> Self {
        Self {
            lock: SpinLock::new(),
            data: null_mut(),
            size: 0,
        }
    }

    /// Creates a new bit map from an exising buffer.
    pub const unsafe fn from_ptr(data: *mut u8, size: usize) -> Self {
        Self {
            lock: SpinLock::new(),
            data,
            size,
        }
    }

    /// Fills the entire map with a single value.
    pub fn fill(&mut self, value: bool) {
        self.lock.acquire_force();
        let byte_val = if value { 0xFF } else { 0 };
        for byte in 0..self.size {
            unsafe {
                *self.data.add(byte) = byte_val;
            }
        }
        self.lock.free();
    }

    /// Sets a bit at `index`.
    pub fn set(&mut self, index: usize, value: bool) {
        self.lock.acquire_force();

        // If the index is out of range, don't do anything.
        if index / 8 >= self.size {
            self.lock.free();
            return;
        }

        // If the bit map was not initialized, don't do anything.
        if self.data.is_null() {
            self.lock.free();
            return;
        }

        unsafe {
            let addr = self.data.add(index / 8);
            let offset = 1 << (index) % 8;

            if value {
                // If we want the value to be true, just OR the bits.
                *addr |= offset;
            } else {
                // If we want the value to be false, mask the bit off.
                *addr &= !offset;
            }
        }

        self.lock.free();
    }

    /// Gets a bit at `index`.
    pub fn get(&mut self, index: usize) -> bool {
        self.lock.acquire_force();

        // If the index is out of range, don't do anything.
        if index / 8 >= self.size {
            self.lock.free();
            return false;
        }

        // If the bit map was not initialized, don't do anything.
        if self.data.is_null() {
            self.lock.free();
            return false;
        }

        let result;
        unsafe {
            result = (*self.data.add(index / 8) & (1 << (index) % 8)) != 0;
        }

        self.lock.free();
        return result;
    }
}
