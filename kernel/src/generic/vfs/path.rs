use alloc::{string::String, vec::Vec};
use core::fmt::{Display, Formatter};

/// Represents an owned file system path.
#[derive(Debug)]
pub struct PathBuf(Vec<u8>);

impl PathBuf {
    /// Creates a new path pointing to the root.
    pub fn new_root() -> Self {
        PathBuf(vec![b'/'])
    }

    /// Creates a new owned path from a string reference.
    pub fn from_str(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }

    /// Creates a new path from a buffer.
    /// # Safety
    /// The caller must ensure that the buffer contains a legal path.
    pub unsafe fn from_unchecked(value: Vec<u8>) -> Self {
        PathBuf(value)
    }

    /// Returns true if the path contained is an absolute path.
    pub fn is_absolute(&self) -> bool {
        self.0.get(0).is_some_and(|&x| x == b'/')
    }
}

impl Display for PathBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8_lossy(&self.0))
    }
}
