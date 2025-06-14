use super::{MountFlags, SuperBlock};
use crate::generic::{
    posix::errno::EResult,
    process::Identity,
    util::mutex::Mutex,
    vfs::{
        entry::Entry,
        fs::FileSystem,
        inode::{CommonOps, INode, Mode},
        path::PathBuf,
    },
};
use alloc::sync::Arc;

#[derive(Debug)]
struct TmpFs;

impl FileSystem for TmpFs {
    fn get_name(&self) -> &'static str {
        "tmpfs"
    }

    fn mount(
        &self,
        mount_point: PathBuf,
        flags: MountFlags,
        identity: &Identity,
    ) -> EResult<(Arc<dyn SuperBlock>, Arc<INode>)> {
        let mount_entry = Entry::lookup(None, mount_point, identity)?;

        todo!()
    }
}

#[derive(Debug)]
struct TmpSuper {
    flags: MountFlags,
    mount: Arc<Entry>,
    root: Arc<Entry>,
}

impl SuperBlock for TmpSuper {
    fn unmount(self) -> EResult<()> {
        todo!()
    }

    fn get_mount_point(self: Arc<Self>) -> Arc<Entry> {
        self.mount.clone()
    }

    fn get_root(self: Arc<Self>) -> EResult<Arc<Entry>> {
        Ok(self.root.clone())
    }

    fn sync(self: Arc<Self>) -> EResult<()> {
        // This is a no-op.
        Ok(())
    }

    fn statvfs(self: Arc<Self>) -> EResult<uapi::statvfs> {
        todo!()
    }

    fn create_inode(self: Arc<Self>, mode: Mode) -> EResult<Arc<INode>> {
        todo!()
    }

    fn destroy_inode(self: Arc<Self>, inode: INode) -> EResult<()> {
        todo!()
    }
}

#[derive(Debug)]
struct TmpNode {
    mtime: Mutex<uapi::timespec>,
    atime: Mutex<uapi::timespec>,
    ctime: Mutex<uapi::timespec>,
}

impl CommonOps for TmpNode {
    fn update_time(
        &self,
        _node: &INode,
        mtime: Option<uapi::timespec>,
        atime: Option<uapi::timespec>,
        ctime: Option<uapi::timespec>,
    ) -> EResult<()> {
        if let Some(x) = mtime {
            *self.mtime.lock() = x;
        }
        if let Some(x) = atime {
            *self.atime.lock() = x;
        }
        if let Some(x) = ctime {
            *self.ctime.lock() = x;
        }
        Ok(())
    }

    fn chmod(&self, node: &INode, mode: Mode) -> EResult<()> {
        todo!()
    }

    fn chown(&self, node: &INode, uid: uapi::uid_t, gid: uapi::gid_t) -> EResult<()> {
        todo!()
    }

    fn sync(&self, _node: &INode) -> EResult<()> {
        // This is a no-op.
        Ok(())
    }
}

fn init() {
    super::register_fs(&TmpFs);
}

init_stage! {
    #[depends(crate::arch::INIT_STAGE)]
    #[entails(crate::generic::vfs::VFS_STAGE)]
    TMPFS_INIT: "generic.vfs.tmpfs" => init;
}
