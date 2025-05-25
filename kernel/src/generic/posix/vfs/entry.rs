use super::{inode::INode, path::PathBuf};
use crate::generic::posix::errno::{EResult, Errno};
use alloc::{collections::btree_set::BTreeSet, string::String, sync::Arc};

/// This struct represents an entry in the VFS.
/// It points to a [`node::Node`], which is like
/// Th
#[derive(Debug)]
pub struct Entry {
    /// The name of this entry.
    pub name: String,

    /// The parent of this entry.
    /// A [`None`] value indicates that this entry is the root.
    parent: Option<Arc<Entry>>,

    /// A list of this node's children which have been accessed and cached so far.
    // TODO: Replace with HashSet for better performance.
    children: BTreeSet<Arc<Entry>>,

    /// The underlying [`INode`] this entry is pointing to.
    link: Option<Arc<INode>>,
}

impl Entry {
    pub fn new(name: String, parent: Option<Arc<Self>>, link: Option<Arc<INode>>) -> Arc<Self> {
        debug_assert!(!name.contains('/'));
        debug_assert!(!name.contains('\0'));

        Arc::new(Entry {
            name,
            parent,
            children: BTreeSet::new(),
            link,
        })
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
