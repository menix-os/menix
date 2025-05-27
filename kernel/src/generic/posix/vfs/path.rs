use alloc::{borrow::ToOwned, string::String, vec::Vec};
use core::fmt::{Display, Formatter, Write};

/// Represents an owned file system path.
#[derive(Debug)]
pub struct PathBuf(Vec<u8>);

impl PathBuf {
    pub fn new() -> Self {
        PathBuf(Vec::new())
    }

    pub fn new_root() -> Self {
        PathBuf(vec![b'/'])
    }

    pub unsafe fn from_unchecked(value: Vec<u8>) -> Self {
        PathBuf(value)
    }
}

impl Display for PathBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&String::from_utf8_lossy(&self.0))
    }
}
