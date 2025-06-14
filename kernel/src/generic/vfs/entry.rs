use super::{inode::INode, path::PathBuf};
use crate::generic::{
    posix::errno::{EResult, Errno},
    process::{Identity, sched::Scheduler},
    util::mutex::Mutex,
    vfs::{fs::SuperBlock, inode::NodeOps},
};
use alloc::{sync::Arc, vec::Vec};

/// This struct represents an entry in the VFS.
#[derive(Debug)]
pub struct Entry {
    /// The name of this entry.
    pub name: Vec<u8>,
    /// The underlying [`INode`] this entry is pointing to.
    /// A [`None`] value indicates that this entry is negative.
    node: Mutex<Option<Arc<INode>>>,
    /// The parent of this entry.
    /// A [`None`] value indicates that this entry is the root.
    parent: Option<Arc<Entry>>,
    /// A list of this node's children which have been accessed and cached so far.
    // TODO: Replace with Set for (probably) better performance.
    children: Mutex<Vec<Arc<Entry>>>,
    /// A file system that is mounted on this entry.
    mount: Mutex<Option<Arc<dyn SuperBlock>>>,
}

impl Entry {
    /// Attempts to look up an entry by a full path.
    pub fn lookup(at: Option<Arc<Self>>, path: PathBuf, identity: &Identity) -> EResult<Arc<Self>> {
        let mut current_path = path.inner();
        let root = Scheduler::get_current().get_process();

        if path.is_absolute() {
            // TODO
        } else {
            // TODO
        }

        todo!()
    }

    /// Attempts to look up an entry in a parent entry.
    /// If it wasn't cached yet, this function will do so.
    pub fn lookup_child(parent: Arc<Self>, name: &[u8]) -> EResult<Arc<Self>> {
        // First, check if we already cached the entry in the same directory at some point.
        for child in parent.children.lock().iter() {
            if child.name == name {
                return Ok(child.clone());
            }
        }

        let result = Arc::new(Entry {
            name: name.to_vec(),
            node: Mutex::new(None), // Always negative unless the file system overwrites this.
            parent: Some(parent.clone()),
            children: Mutex::new(Vec::new()),
            mount: Mutex::new(None),
        });

        // Make sure the parent is actually cached.
        let parent_node = parent.node.lock().clone().ok_or(Errno::ENOENT)?;

        // Do a lookup in the file system.
        let NodeOps::Directory(dir) = &parent_node.node_ops else {
            return Ok(result);
        };
        dir.lookup(&parent_node, &result)?;

        // Lookup successful, save the child.
        parent.children.lock().push(result.clone());

        return Ok(result);
    }

    /// Searches the root entry where the file system of this entry is mounted on.
    pub fn get_mount_top(self: Arc<Self>) -> Arc<Self> {
        let mut current = self;

        // TODO?
        loop {
            let mount = current.mount.lock().clone();
            match mount {
                Some(x) => current = x.get_mount_point(),
                None => break,
            }
        }

        return current;
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
