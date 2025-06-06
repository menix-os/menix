use super::{inode::INode, path::PathBuf};
use crate::generic::posix::errno::{EResult, Errno};
use alloc::{collections::btree_set::BTreeSet, sync::Arc, vec::Vec};

#[derive(Debug)]
enum EntryKind {
    /// The node points to a valid inode.
    Positive(Arc<INode>),
    /// The node points to an invalid inode. This is usually the case when a lookup has failed.
    Negative,
    /// The node hasn't been cached yet.
    Unknown,
}

/// This struct represents an entry in the VFS.
#[derive(Debug)]
pub struct Entry {
    /// The name of this entry.
    pub name: Vec<u8>,

    /// The underlying [`INode`] this entry is pointing to.
    /// A [`None`] value indicates that this entry is negative.
    node: EntryKind,

    /// The parent of this entry.
    /// A [`None`] value indicates that this entry is the root.
    parent: Option<Arc<Entry>>,

    /// A list of this node's children which have been accessed and cached so far.
    // TODO: Replace with HashSet for (probably) better performance.
    children: BTreeSet<Arc<Entry>>,
}

impl Entry {
    /// Creates a new VFS entry
    pub fn new(name: Vec<u8>, parent: Option<Arc<Self>>) -> Arc<Self> {
        assert!(
            !name.contains(&b'/') && !name.contains(&0),
            "Entries should not contain forward slashes or NUL terminators!"
        );
        Arc::new(Entry {
            name,
            parent,
            children: BTreeSet::new(),
            node: EntryKind::Unknown,
        })
    }

    /// Attempts to get an inode. If the inode doesn't exist, returns [`None`].
    /// If it wasn't cached yet, it will get cached.
    pub fn get_inode(&self) -> Option<Arc<INode>> {
        match &self.node {
            EntryKind::Positive(inode) => Some(inode.clone()),
            EntryKind::Negative => None,
            EntryKind::Unknown => {
                todo!();
            }
        }
    }

    /// Returns the absolute path to this entry.
    pub fn get_path(self: &Arc<Self>) -> EResult<PathBuf> {
        // If there are no parent nodes, we're already at the root.
        if self.parent.is_none() {
            return Ok(PathBuf::new_root());
        }

        let mut result = vec![0u8; uapi::PATH_MAX as usize];
        let mut offset = uapi::PATH_MAX as usize;
        let mut current = self;

        while let Some(parent) = &current.parent {
            let len = current.name.len();
            offset = offset.checked_sub(len + 1).ok_or(Errno::ENAMETOOLONG)?;
            result[offset + 1..][..len].copy_from_slice(&current.name);
            result[offset] = b'/';

            current = parent;
        }

        Ok(unsafe { PathBuf::from_unchecked(result[offset..].to_vec()) })
    }
}
