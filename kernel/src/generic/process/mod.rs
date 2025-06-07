pub mod sched;
pub mod task;

use crate::generic::{
    memory::{
        VirtAddr,
        pmm::FreeList,
        virt::{KERNEL_PAGE_TABLE, PageTable},
    },
    posix::errno::{EResult, Errno},
    resource::Resource,
    vfs::{exec::ExecutableInfo, path::PathBuf},
};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use core::sync::atomic::{AtomicUsize, Ordering};
use task::Task;

/// A unique process ID.
pub type Pid = usize;

/// Represents a process and address space.
#[derive(Debug)]
pub struct Process {
    /// The unique identifier of this process.
    id: Pid,
    name: String,
    pub page_table: PageTable,
    threads: Vec<Task>,
    is_user: bool,
}

#[derive(Debug)]
pub struct Identity {
    pub user_id: uapi::uid_t,
    pub group_id: uapi::gid_t,

    pub effective_user_id: uapi::uid_t,
    pub effective_group_id: uapi::gid_t,

    pub set_user_id: uapi::uid_t,
    pub set_group_id: uapi::gid_t,

    pub groups: Vec<uapi::gid_t>,
}

static PID_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Process {
    /// Returns the unique identifier of this process.
    #[inline]
    pub const fn get_pid(&self) -> Pid {
        self.id
    }

    /// Returns true if this is a user process.
    #[inline]
    pub const fn is_user(&self) -> bool {
        self.is_user
    }

    pub fn new(name: String, is_user: bool) -> Self {
        Self {
            id: PID_COUNTER.fetch_add(1, Ordering::Relaxed),
            name,
            page_table: PageTable::new_user::<FreeList>(KERNEL_PAGE_TABLE.lock().root_level()),
            threads: Vec::new(),
            is_user,
        }
    }

    /// Creates a new user process from a resource. It determines the execution format by reading the first few bytes.
    pub fn from_file(path: &PathBuf) -> EResult<Self> {
        let res: Box<dyn Resource> = todo!();

        // Peek into the resource and read the magic. 4 bytes are enough to fit the longest magic.
        let mut magic = [0u8; 4];
        res.read(0, &mut magic).unwrap();

        let info = ExecutableInfo {
            executable: res,
            interpreter: None,
        };

        match &magic[0..] {
            // This is an ELF executable.
            b"\x7fELF" => {
                log!("It's an elf!")
            }
            // This is a script.
            b"#!" => {}
            // No idea what this format is.
            _ => {
                error!("Unknown binary format in file \"{}\"", path);
                return Err(Errno::ENOEXEC);
            }
        }
        let mut result = Self::new(todo!(), true);

        let main_thread = Task::new(to_user, todo!(), todo!(), Some(Arc::new(result)), true);

        return Ok(result);
    }
}

pub extern "C" fn to_user(ip: usize, sp: usize) {
    unsafe { crate::arch::sched::jump_to_user(VirtAddr::from(ip), VirtAddr::from(sp)) };
}
