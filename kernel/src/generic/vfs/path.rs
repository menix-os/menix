use crate::generic::{
    posix::errno::{EResult, Errno},
    process::sched::Scheduler,
    vfs::entry::{Entry, Mount},
};
use alloc::sync::Arc;
use core::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Path {
    pub mount: Arc<Mount>,
    pub entry: Arc<Entry>,
}

impl Path {
    /// Creates a new path from a string path, relative to the working directory of the current process.
    pub fn new(value: &[u8]) -> EResult<Self> {
        let proc = Scheduler::get_current().get_process();
        let cwd = proc.working_dir.lock();

        if *value.get(0).ok_or(Errno::EINVAL)? == b'/' {}

        todo!()
    }

    /// Looks up a child identified by `name`.
    pub fn lookup_child(self, name: &[u8]) -> Path {
        let mut mount = self.mount.as_ref();
        let mut entry = self.entry.as_ref();

        todo!()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(todo!())
    }
}
