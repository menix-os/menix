use super::inode::INode;
use crate::generic::{
    posix::errno::{EResult, Errno},
    process::{Identity, sched::Scheduler},
    util::mutex::Mutex,
    vfs::{File, file::OpenFlags, fs::Mount, inode::NodeOps},
};
use alloc::{collections::btree_map::BTreeMap, sync::Arc, vec::Vec};
use core::hint::unlikely;

#[derive(Debug, Default)]
pub enum EntryState {
    /// Entry is positive and contains a link to the inode.
    Present(Arc<INode>),
    /// Entry is negative.
    NotPresent,
    /// The entry hasn't been looked up yet.
    #[default]
    NotCached,
}

/// This struct represents an entry in the VFS.
#[derive(Debug)]
pub struct Entry {
    /// The name of this entry.
    pub name: Vec<u8>,
    /// The underlying [`INode`] this entry is pointing to.
    pub inode: Mutex<EntryState>,
    /// The parent of this [`Entry`].
    /// A [`None`] value indicates that this entry is a root.
    pub parent: Option<Arc<Entry>>,
    /// If the [`Self::present`] is set to `true`,
    /// then this contains a map of all children of this entry.
    pub children: Mutex<BTreeMap<Vec<u8>, Arc<Entry>>>,
    /// A list of mounts on this entry.
    pub mounts: Mutex<Vec<Arc<Mount>>>,
}

impl Entry {
    pub fn new(name: &[u8], inode: Option<Arc<INode>>, parent: Option<Arc<Entry>>) -> Self {
        Entry {
            name: name.to_vec(),
            inode: Mutex::new(match inode {
                Some(x) => EntryState::Present(x),
                None => EntryState::NotPresent,
            }),
            parent,
            children: Mutex::new(BTreeMap::new()),
            mounts: Mutex::new(Vec::new()),
        }
    }

    pub fn get_inode(&self) -> Option<Arc<INode>> {
        let mut lock = self.inode.lock();
        match &*lock {
            EntryState::Present(inode) => Some(inode.clone()),
            EntryState::NotPresent => None,
            EntryState::NotCached => {
                // Do lookup if it wasn't cached already.
                *lock = EntryState::NotPresent;
                todo!("Lookup inode and cache")
            }
        }
    }

    pub fn set_inode(&self, inode: Arc<INode>) {
        *self.inode.lock() = EntryState::Present(inode);
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

            current_node = current_node.resolve_symlink(identity, flags)?;
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

        if flags.contains(LookupFlags::FollowSymlinks) {
            current_node = current_node.resolve_symlink(identity, flags)?;
        }

        let inode = current_node.entry.get_inode();
        if flags.contains(LookupFlags::MustExist) && inode.is_none() {
            return Err(Errno::ENOENT);
        }
        if flags.contains(LookupFlags::MustNotExist) && inode.is_some() {
            return Err(Errno::EEXIST);
        }
        return Ok(current_node);
    }

    pub fn lookup_child(self, name: &[u8]) -> EResult<Self> {
        // Traverse the mounts.
        let mut mount = self.mount.clone();
        let mut entry = self.entry.clone();

        'again: loop {
            for child_mnt in entry.clone().mounts.lock().iter() {
                if Arc::ptr_eq(&child_mnt, &mount) {
                    mount = child_mnt.clone();
                    entry = child_mnt.root.clone();
                    continue 'again;
                }
            }
            break;
        }

        // If this entry has already been looked up before, return that.
        if let Some(child) = entry.children.lock().get(name) {
            return Ok(Self {
                mount,
                entry: child.clone(),
            });
        }

        // If it hasn't, we have to perform a new lookup into the file system.
        let parent = entry
            .get_inode()
            .expect("This directory didn't contain an inode");

        // A lookup only makes sense on directories.
        let NodeOps::Directory(x) = &parent.node_ops else {
            return Err(Errno::ENOTDIR);
        };

        let child = PathNode {
            mount,
            entry: Arc::try_new(Entry {
                name: name.to_vec(),
                inode: Mutex::default(),
                parent: Some(entry.clone()),
                children: Mutex::default(),
                mounts: Mutex::default(),
            })?,
        };

        if let Err(e) = x.lookup(&parent, &child) {
            // Allow lookup failures so we can cache it as a negative entry.
            if e != Errno::ENOENT {
                return Err(e);
            }
            *child.entry.inode.lock() = EntryState::NotPresent;
        }

        // Insert the new entry as a child.
        entry
            .children
            .lock()
            .insert(name.to_vec(), child.entry.clone());

        return Ok(child);
    }

    pub fn lookup_parent(&self) -> EResult<Self> {
        // Get the top mount point.
        let mut mount = self.mount.clone();
        let mut entry = self.entry.clone();
        while let Some(mount_point) = mount.clone().mount_point.lock().as_ref()
            && Arc::ptr_eq(&entry, &mount.root)
        {
            mount = mount_point.mount.clone();
            entry = mount_point.entry.clone();
        }

        return Ok(Self {
            mount,
            entry: entry.parent.clone().ok_or(Errno::ENOENT)?,
        });
    }

    fn resolve_symlink(&self, identity: &Identity, flags: LookupFlags) -> EResult<Self> {
        let mut link_buf = vec![0u8; uapi::PATH_MAX as _];
        let mut current = self.clone();
        while let Some(inode) = current.entry.get_inode()
            && let NodeOps::SymbolicLink(symlink) = &inode.node_ops
        {
            let parent = current.entry.parent.as_ref().expect("Should have a root");
            let link_length = symlink.read_link(&inode, &mut link_buf)? as usize;

            let result = Self::lookup(
                Some(PathNode {
                    mount: self.mount.clone(),
                    entry: parent.clone(),
                }),
                &link_buf[0..link_length],
                identity,
                flags,
            )?;

            if Arc::ptr_eq(&result.entry, &current.entry) {
                return Err(Errno::ELOOP);
            }
            current = result;
        }

        return Ok(current);
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
    #[derive(Debug, Clone, Copy)]
    pub struct LookupFlags: u32 {
        const FollowSymlinks = 1 << 0;
        const MustExist = 1 << 1;
        const MustNotExist = 1 << 2;
        const UseRealId = 1 << 3;
    }
}

/// A cached and/or mapped object.
#[derive(Debug)]
pub struct AddressSpace {
    pub owner: Arc<INode>,
}
