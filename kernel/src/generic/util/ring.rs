use alloc::vec::Vec;
use core::cmp::min;

#[derive(Debug)]
pub struct RingBuffer {
    buf: Vec<u8>,
    read_cursor: usize,
    write_cursor: usize,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: vec![0u8; capacity],
            read_cursor: 0,
            write_cursor: 0,
        }
    }

    /// Reads bytes from the ring to a buffer.
    /// Returns the amount of bytes read
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let len = self.peek(buf);
        self.read_cursor = (self.read_cursor + len) % self.capacity();

        len
    }

    /// Writes bytes from a buffer to the ring.
    /// Returns the amount of bytes written.
    pub fn write(&mut self, buf: &[u8]) -> usize {
        let cursor = self.write_cursor;
        let len = min(buf.len(), self.get_available_len());
        let capacity = self.capacity();
        let buffer = self.inner();

        let l0 = min(cursor + len, capacity) - cursor;
        buffer[cursor..(cursor + l0)].copy_from_slice(&buf[0..l0]);

        let l1 = len - l0;
        if l1 > 0 {
            buffer[0..l1].copy_from_slice(&buf[l0..(l0 + l1)]);
        }

        self.write_cursor = (self.write_cursor + len) % capacity;

        len
    }

    pub fn peek(&mut self, buf: &mut [u8]) -> usize {
        let cursor = self.read_cursor;
        let len = min(buf.len(), self.get_data_len());
        let capacity = self.capacity();
        let buffer = self.inner();

        let l0 = min(cursor + len, capacity) - cursor;
        buf[0..l0].copy_from_slice(&buffer[cursor..(cursor + l0)]);

        let l1 = len - l0;
        if l1 > 0 {
            buf[l0..(l0 + l1)].copy_from_slice(&buffer[0..l1]);
        }

        len
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.read_cursor == self.write_cursor
    }

    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.get_available_len() == 0
    }

    pub fn get_data_len(&self) -> usize {
        if self.read_cursor <= self.write_cursor {
            self.write_cursor - self.read_cursor
        } else {
            self.capacity() - (self.read_cursor - self.write_cursor)
        }
    }

    #[inline(always)]
    pub fn get_available_len(&self) -> usize {
        self.capacity() - self.get_data_len() - 1
    }

    #[inline(always)]
    fn inner(&mut self) -> &mut [u8] {
        self.buf.as_mut()
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.read_cursor = 0;
        self.write_cursor = 0;
    }
}
