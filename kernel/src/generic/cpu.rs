// Per-CPU data structures.

use super::{
    memory::{PageAlloc, VirtAddr},
    sched::thread::Thread,
};
use crate::arch::{self};
use alloc::{boxed::Box, sync::Arc};
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::RwLock;

/// Common processor-local information.
#[derive(Debug)]
pub struct CpuData {
    /// The ID of this CPU.
    pub id: usize,
    /// Stack pointer for kernel mode. Only used for task switching.
    pub kernel_stack: VirtAddr,
    /// Stack pointer for user mode.
    pub user_stack: VirtAddr,
    /// Whether this CPU is online.
    pub online: bool,
    /// Whether this CPU is present.
    pub present: bool,
    /// Current thread running on this CPU.
    pub thread: Option<Arc<RwLock<Thread>>>,
}

impl CpuData {
    pub fn get() -> &'static mut CpuData {
        return unsafe { arch::cpu::get_per_cpu().as_mut().unwrap() };
    }
}

/// [`PerCpuData`] uses the following trick: All data is placed into a special section.
/// At first, this section only contains one instance of all fields (enough for the boot CPU and
/// to trick the borrow checker). When a new CPU is allocated, the contents of this section are copied over,
/// and the memory region is extended. This is why the per-CPU section *must* always come last.
/// The actual access to this variable is done by calculating the offset of the current per-CPU context.
#[repr(transparent)]
pub struct PerCpuData<T: 'static> {
    /// This value is only used as placeholder storage. It is never directly accessed.
    storage: T,
}

unsafe impl<T> Send for PerCpuData<T> {}
unsafe impl<T> Sync for PerCpuData<T> {}

impl<T> PerCpuData<T> {
    pub const fn new(value: T) -> Self {
        return Self { storage: value };
    }

    pub fn get(&self, _context: &CpuData) -> &'static mut T {
        return unsafe { (arch::cpu::get_per_cpu() as *mut T).as_mut().unwrap() };
    }
}

#[macro_export]
macro_rules! per_cpu {
    ($name:ident, $ty:ty, $value:expr) => {
        #[unsafe(link_section = ".percpu")]
        pub static $name: $crate::generic::cpu::PerCpuData<$ty> =
            $crate::generic::cpu::PerCpuData::new($value);
    };
}

per_cpu!(
    CPU_DATA,
    CpuData,
    CpuData {
        id: 0,
        kernel_stack: VirtAddr(0),
        user_stack: VirtAddr(0),
        online: false,
        present: false,
        thread: None,
    }
);

/// Counts how many CPUs have been allocated.
static CPU_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Sets up the boot CPU.
#[deny(dead_code)]
pub(crate) fn setup_bsp() {}

/// Extends the per-CPU data for a new CPU.
/// Returns the new CpuData context and the new CPU ID.
pub(crate) fn allocate_cpu() -> (&'static mut CpuData, usize) {
    let id = CPU_COUNTER.fetch_add(1, Ordering::Relaxed);
    let data = CpuData {
        id,
        kernel_stack: VirtAddr(0),
        user_stack: VirtAddr(0),
        online: false,
        present: false,
        thread: None,
    };
    let memory = Box::leak(Box::new_in(data, PageAlloc));

    return (memory, id);
}
