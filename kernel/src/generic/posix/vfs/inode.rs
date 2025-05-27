use super::{fs::SuperBlock, path::PathBuf};
use crate::generic::{posix::errno::EResult, util::mutex::Mutex};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{fmt::Debug, sync::atomic::AtomicBool};

/// A standalone inode. See [`super::entry::Entry`] for information.
#[derive(Debug)]
pub struct INode {
    /// FS-specific callbacks that can be performed on this node.
    pub ops: Box<dyn NodeOps>,
    /// The super block which this node is located in.
    pub sb: Arc<SuperBlock>,
    /// The status of this node.
    pub stat: Mutex<Stat>,
    /// If true, the node has been modified and has to be sync'd.
    pub dirty: AtomicBool,
}

impl INode {
    pub fn get_stat(&self) -> Stat {
        self.stat.lock().clone()
    }

    pub fn read_symlink(&self) -> EResult<PathBuf> {
        let mut result = Vec::new();
        let mut len = 0;
        while len < uapi::SYMLINK_MAX as usize {
            len = self.ops.read_symlink(self, &mut result)?;
        }

        Ok(unsafe { PathBuf::from_unchecked(result) })
    }
}

/// Operations which work on a node.
pub trait NodeOps: Debug {
    /// Looks up an entry in the directory.
    fn lookup(&self, dir: &INode) -> EResult<()>;

    /// Synchronizes the node back to the underlying file system.
    fn sync(&self, node: &INode) -> EResult<()>;

    /// Reads the path of the symbolic link of the node into a buffer.
    fn read_symlink(&self, node: &INode, out: &mut Vec<u8>) -> EResult<usize>;
}

#[derive(Debug, Clone)]
pub struct Stat {
    inner: uapi::stat,
}

impl Stat {
    pub fn new() -> Self {
        Self {
            inner: uapi::stat {
                st_dev: 0,
                st_ino: 0,
                st_mode: uapi::S_IROTH | uapi::S_IRGRP | uapi::S_IRUSR,
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

    pub fn get_file_type(&self) -> FileType {
        match self.inner.st_mode & uapi::S_IFMT {
            uapi::S_IFREG => FileType::Regular,
            uapi::S_IFBLK => FileType::BlockDevice,
            uapi::S_IFCHR => FileType::CharacterDevice,
            uapi::S_IFIFO => FileType::FIFO,
            uapi::S_IFSOCK => FileType::Socket,
            uapi::S_IFDIR => FileType::Directory,
            uapi::S_IFLNK => FileType::SymbolicLink,
            _ => panic!("Impossible file type in mode {:#x}", self.inner.st_mode),
        }
    }
}

pub enum FileType {
    Regular,
    Directory,
    SymbolicLink,
    FIFO,
    BlockDevice,
    CharacterDevice,
    Socket,
}
