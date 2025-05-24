use super::{node::Node, path::PathBuf};
use crate::generic::posix::errno::{EResult, Errno};
use alloc::{string::String, sync::Arc, vec::Vec};

/// A cached directory entry as part of the VFS.
#[derive(Debug)]
pub struct Entry {
    /// The name of this entry.
    pub name: String,
    /// A [`None`] value indicates that this entry is the root.
    pub parent: Option<Arc<Entry>>,
    /// A list of this node's children which have been accessed and cached so far.
    children: Vec<Arc<Entry>>,
    node: Option<Arc<Node>>,
}

impl Entry {
    pub fn new(name: String, parent: Option<Arc<Self>>) -> EResult<Arc<Self>> {
        if name.contains('/') || name.contains('\0') {
            return Err(Errno::EINVAL);
        }

        Ok(Arc::new(Entry {
            name,
            parent,
            children: Vec::new(),
            node: None,
        }))
    }

    /// Returns the absolute path to this entry.
    pub fn get_path(this: &Arc<Self>) -> EResult<PathBuf> {
        // If there are no parent nodes, we're already at the root.
        if this.parent.is_none() {
            return Ok(PathBuf::new_root());
        }

        let mut result = String::with_capacity(uapi::PATH_MAX as usize);
        let mut current = this;

        while let Some(parent) = &current.parent {
            result.insert_str(0, &current.name);
            result.insert(0, '/');

            // If the string is too long, abort.
            if result.len() > uapi::PATH_MAX as usize {
                return Err(Errno::ENAMETOOLONG);
            }

            current = parent;
        }

        Ok(unsafe { PathBuf::from_string_unchecked(String::from(result)) })
    }
}
