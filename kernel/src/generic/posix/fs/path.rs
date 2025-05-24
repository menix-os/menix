use core::fmt::Display;

use alloc::string::{String, ToString};

/// Represents an owned file system path.
#[derive(Debug)]
pub struct PathBuf(String);

impl PathBuf {
    pub fn new() -> Self {
        PathBuf(String::new())
    }

    pub fn new_root() -> Self {
        PathBuf(b'/'.to_string())
    }

    pub unsafe fn from_string_unchecked(string: String) -> Self {
        PathBuf(string)
    }
}

impl Display for PathBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}
