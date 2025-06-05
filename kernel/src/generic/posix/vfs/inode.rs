use super::{fs::SuperBlock, path::PathBuf};
use crate::generic::{
    posix::{
        errno::EResult,
        vfs::{entry::Entry, file::FileOps},
    },
    util::mutex::Mutex,
};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{fmt::Debug, sync::atomic::AtomicBool};

/// A standalone inode. See [`super::entry::Entry`] for information.
#[derive(Debug)]
pub struct INode {
    /// FS-specific callbacks that can be performed on this node.
    pub node_ops: Box<dyn NodeOps>,
    pub file_ops: Box<dyn FileOps>,

    /// The super block which this node is located in.
    pub sb: Arc<dyn SuperBlock>,

    /// The status of this node.
    pub stat: Mutex<Stat>,

    /// If true, the node has been modified and has to be sync'd.
    pub dirty: AtomicBool,
}

impl INode {
    pub fn new(
        node_ops: Box<dyn NodeOps>,
        file_ops: Box<dyn FileOps>,
        sb: Arc<dyn SuperBlock>,
        mode: uapi::mode_t,
    ) -> EResult<Self> {
        Ok(Self {
            node_ops,
            file_ops,
            sb,
            stat: Mutex::new(Stat::from_mode(mode)?),
            dirty: AtomicBool::new(false),
        })
    }

    pub fn get_stat(&self) -> Stat {
        self.stat.lock().clone()
    }

    pub fn read_symlink(&self) -> EResult<PathBuf> {
        let mut result = Vec::new();
        let mut len = 0;
        while len < uapi::SYMLINK_MAX as usize {
            len = self.node_ops.read_symlink(self, &mut result)?;
        }

        Ok(unsafe { PathBuf::from_unchecked(result) })
    }
}

/// Operations which work on a node.
pub trait NodeOps: Debug {
    /// Attempts to resolve an `entry` in a given `directory`.
    /// If a node is found, the target node is set on `entry`.
    /// If it isn't found, the entry is marked negative and [`Errno::ENOENT`] is returned.
    fn lookup(&self, entry: &Entry, directory: &INode) -> EResult<()>;

    /// Synchronizes the node back to the underlying file system.
    fn sync(&self, node: &INode) -> EResult<()>;

    /// Reads the path of the symbolic link of the node into a buffer.
    fn read_symlink(&self, node: &INode, out: &mut [u8]) -> EResult<usize>;
}

pub enum NodeType {
    Regular,
    Directory,
    SymbolicLink,
    FIFO,
    BlockDevice,
    CharacterDevice,
    Socket,
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Stat {
    inner: uapi::stat,
}

impl Stat {
    pub fn new() -> Self {
        Self {
            inner: uapi::stat {
                st_dev: 0,
                st_ino: 0,
                st_mode: uapi::S_IFREG | uapi::S_IROTH | uapi::S_IRGRP | uapi::S_IRUSR,
                st_nlink: 1,
                st_uid: 0,
                st_gid: 0,
                st_rdev: 0,
                st_size: 0,
                st_atim: uapi::timespec::default(),
                st_mtim: uapi::timespec::default(),
                st_ctim: uapi::timespec::default(),
                st_blksize: 0,
                st_blocks: 0,
            },
        }
    }

    /// Sets the bits
    pub fn set_mode(&mut self, mode: uapi::mode_t) {
        self.inner.st_mode = mode & !uapi::S_IFMT;
    }

    pub fn from_mode(mode: uapi::mode_t) -> EResult<Self> {
        let mut result = Self::new();
        result.inner.st_mode = mode;
        return Ok(result);
    }

    pub fn get_file_type(&self) -> NodeType {
        match self.inner.st_mode & uapi::S_IFMT {
            uapi::S_IFREG => NodeType::Regular,
            uapi::S_IFBLK => NodeType::BlockDevice,
            uapi::S_IFCHR => NodeType::CharacterDevice,
            uapi::S_IFIFO => NodeType::FIFO,
            uapi::S_IFSOCK => NodeType::Socket,
            uapi::S_IFDIR => NodeType::Directory,
            uapi::S_IFLNK => NodeType::SymbolicLink,
            _ => panic!("Impossible file type in mode {:#x}", self.inner.st_mode),
        }
    }
}
