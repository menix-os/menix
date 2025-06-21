use super::inode::INode;
use crate::generic::{
    posix::errno::{EResult, Errno},
    process::{Identity, sched::Scheduler},
    util::mutex::Mutex,
    vfs::{File, file::OpenFlags, fs::Mount, inode::NodeOps},
};
use alloc::{
    collections::btree_map::BTreeMap,
    sync::{Arc, Weak},
    vec::Vec,
};
use core::{
    hint::unlikely,
    sync::atomic::{AtomicBool, Ordering},
};

/// This struct represents an entry in the VFS.
#[derive(Debug)]
pub struct Entry {
    /// The name of this entry.
    pub name: Vec<u8>,
    /// Whether this entry has a backing node.
    pub present: AtomicBool,
    /// The underlying [`INode`] this entry is pointing to.
    /// A [`None`] value indicates that this entry is negative.
    pub inode: Mutex<Option<Arc<INode>>>,
    /// The parent of this [`Entry`].
    /// A [`None`] value indicates that this entry is a root.
    pub parent: Option<Arc<Entry>>,
    /// If the [`Self::present`] is set to `true`,
    /// then this contains a map of all children of this entry.
    pub children: Mutex<BTreeMap<Vec<u8>, Weak<Entry>>>,
    /// A list of mounts on this entry.
    pub mounts: Mutex<Vec<Weak<Mount>>>,
}

impl Entry {
    pub fn new(name: &[u8], inode: Option<Arc<INode>>, parent: Option<Arc<Entry>>) -> Self {
        Entry {
            name: name.to_vec(),
            present: AtomicBool::new(inode.is_some()),
            inode: Mutex::new(inode),
            parent,
            children: Mutex::new(BTreeMap::new()),
            mounts: Mutex::new(Vec::new()),
        }
    }

    pub fn get_inode(&self) -> Option<Arc<INode>> {
        if self.present.load(Ordering::Acquire) {
            return self.inode.lock().clone();
        }

        // Do lookup if it wasn't cached already.
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct PathNode {
    pub mount: Arc<Mount>,
    pub entry: Arc<Entry>,
}

impl PathNode {
    pub fn flookup(
        file: Option<Arc<File>>,
        path: &[u8],
        identity: &Identity,
        flags: LookupFlags,
    ) -> EResult<Self> {
        let start = match file {
            Some(x) => match x.path.clone() {
                Some(p) => Some(p),
                None => return Err(Errno::ENOENT),
            },
            None => None,
        };
        return Self::lookup(start, path, identity, flags);
    }

    pub fn lookup(
        start: Option<Self>,
        path: &[u8],
        identity: &Identity,
        flags: LookupFlags,
    ) -> EResult<Self> {
        if unlikely(path.is_empty()) {
            return Err(Errno::ENOENT);
        }

        let proc = Scheduler::get_current().get_process();

        // If a path starts with '/', it's an absolute path.
        // In that case, skip the first character and use the current root as a starting point.
        let (mut current_node, path) = if path.get(0).is_some_and(|&x| x == b'/') {
            (proc.root_dir.lock().clone(), &path[1..])
        } else {
            (start.unwrap_or(proc.working_dir.lock().clone()), path)
        };

        // Parse each component.
        for component in path.split(|&x| x == b'/').filter(|&x| !x.is_empty()) {
            // A path may never contain a NUL terminator.
            if unlikely(component.contains(&0)) {
                return Err(Errno::EILSEQ);
            }

            // TODO: Resolve symlinks.

            let Some(inode) = current_node.entry.get_inode() else {
                return Err(Errno::ENOENT);
            };

            let NodeOps::Directory(_) = &inode.node_ops else {
                return Err(Errno::ENOTDIR);
            };

            current_node.entry.get_inode().unwrap().try_access(
                identity,
                OpenFlags::empty(),
                flags.contains(LookupFlags::UseRealId),
            )?;
            current_node = current_node.lookup_child(component)?;
        }

        return Ok(current_node);
    }

    pub fn lookup_child(self, name: &[u8]) -> EResult<Self> {
        if let Some(child) = self.entry.children.lock().get(name) {
            return Ok(child
                .upgrade()
                .map(|x| Self {
                    mount: self.mount.clone(),
                    entry: x,
                })
                .expect("Child node should have been accessible"));
        }

        let parent = self
            .entry
            .get_inode()
            .expect("This directory didn't contain an inode");
        let NodeOps::Directory(x) = &parent.node_ops else {
            return Err(Errno::ENOTDIR);
        };

        let mut child = Entry {
            name: name.to_vec(),
            present: AtomicBool::new(false),
            inode: Mutex::default(),
            parent: Some(self.entry.clone()),
            children: Mutex::default(),
            mounts: Mutex::default(),
        };

        x.lookup(&parent, &mut child)?;

        let child_arc = Arc::try_new(child)?;

        self.entry
            .children
            .lock()
            .insert(name.to_vec(), Arc::downgrade(&child_arc));

        return Ok(PathNode {
            mount: self.mount,
            entry: child_arc,
        });
    }

    /// Traverses a path until it encounters a node with no mount point.
    pub fn get_mount_top(self) -> PathNode {
        let mut current = self;

        loop {
            let root = match &*current.mount.mount_point.lock() {
                Some(x) => x.clone(),
                None => break,
            };
            current = root;
        }

        current
    }
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct LookupFlags: u32 {
        const FollowSymlinks = 1 << 0;
        const MustExist = 1 << 1;
        const MustNotExist = 1 << 2;
        const UseRealId = 1 << 3;
    }
}
