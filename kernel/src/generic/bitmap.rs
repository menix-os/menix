// Bit map storage

use core::ptr::{null_mut, slice_from_raw_parts_mut};
use spin::mutex::Mutex;

#[derive(Debug)]
pub struct BitMap<'a> {
    /// Storage backing this bit map.
    data: Mutex<Option<&'a mut [u8]>>,
}

impl BitMap<'_> {
    /// Creates a new empty bit map.
    pub const fn new() -> Self {
        Self {
            data: Mutex::new(None),
        }
    }

    /// Creates a new bit map from an exising buffer.
    pub unsafe fn from_ptr(data: *mut u8, size: usize) -> Self {
        Self {
            data: unsafe { Mutex::new(slice_from_raw_parts_mut(data, size).as_mut()) },
        }
    }

    /// Fills the entire map with a single value.
    pub fn fill(&mut self, value: bool) {
        match self.data.lock().as_deref_mut() {
            Some(x) => {
                let byte_val = if value { 0xFF } else { 0 };
                for byte in x {
                    *byte = byte_val;
                }
            }
            None => panic!("Bit map was not initialized!"),
        }
    }

    /// Sets a bit at `index`.
    pub fn set(&mut self, index: usize, value: bool) {
        // If the bit map was not initialized, don't do anything.
        match self.data.lock().as_deref_mut() {
            Some(x) => {
                let addr = match x.get_mut(index / 8) {
                    Some(idx) => idx,
                    None => return,
                };
                let offset = 1 << (index) % 8;

                if value {
                    // If we want the value to be true, just OR the bits.
                    *addr |= offset;
                } else {
                    // If we want the value to be false, mask the bit off.
                    *addr &= !offset;
                }
            }
            None => panic!("Bit map was not initialized!"),
        }
    }

    /// Gets a bit at `index`.
    pub fn get(&self, index: usize) -> Option<bool> {
        match self.data.lock().as_deref_mut() {
            Some(x) => {
                match x.get(index / 8) {
                    Some(idx) => {
                        let result = *idx & (1 << (index) % 8) != 0;
                        return Some(result);
                    }
                    None => return None,
                };
            }
            None => return None,
        }
    }
}
