// Per-CPU data structures.

use super::{
    memory::VirtAddr,
    sched::{Scheduler, task::Task},
};
use crate::arch::{self};
use alloc::{boxed::Box, sync::Arc};
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::RwLock;

/// Common processor-local information.
#[derive(Debug)]
pub struct CpuData {
    /// A pointer to this exact structure.
    pub this: *mut CpuData,
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
    pub scheduler: Scheduler,
}

impl CpuData {
    /// Gets the data for the current CPU.
    pub fn get() -> &'static mut CpuData {
        return unsafe { arch::cpu::get_per_cpu().as_mut().unwrap() };
    }

    /// Gets the data for a specific CPU.
    pub unsafe fn get_for(id: usize) -> &'static mut CpuData {
        assert!(
            id < NUM_CPUS.load(Ordering::Relaxed),
            "Attempted to get a per-CPU block for a nonexistent CPU {}!",
            id
        );
        let size = &raw const LD_PERCPU_END as usize - &raw const LD_PERCPU_START as usize;
        let address = &raw const LD_PERCPU_START as usize + (size * id);
        return unsafe { (address as *mut CpuData).as_mut().unwrap() };
    }
}

/// [`PerCpuData`] uses the following trick: All data is placed into a special section.
/// At first, this section only contains one instance of all fields (enough to initialize the boot CPU and
/// to trick the borrow checker). When a new CPU is allocated, the contents of this section are copied over,
/// and the memory region is extended. This is why the per-CPU section *must* always come last.
/// The actual access to this variable is done by calculating the offset of the current per-CPU context.
///
/// Do not use this directly, instead use the [`crate::per_cpu`] macro!
#[repr(transparent)]
pub struct PerCpuData<T: 'static> {
    /// This value is only used as placeholder storage. It is never directly accessed.
    storage: T,
}

// We guarantee that this data is only ever accessed by one CPU.
unsafe impl<T> Send for PerCpuData<T> {}
unsafe impl<T> Sync for PerCpuData<T> {}

unsafe extern "C" {
    pub unsafe static LD_PERCPU_START: u8;
    pub unsafe static LD_PERCPU_END: u8;
}

impl<T> PerCpuData<T> {
    pub const fn new(value: T) -> Self {
        return Self { storage: value };
    }

    pub fn get(&self, context: &CpuData) -> &'static mut T {
        // Calculate the offset into the per-CPU region.
        let size = &raw const LD_PERCPU_END as usize - &raw const LD_PERCPU_START as usize;
        let offset = (&raw const self.storage as usize - &raw const LD_PERCPU_START as usize)
            + (size * context.id);
        unsafe {
            let address = (context.this as *mut T).byte_add(offset);
            return address.as_mut().unwrap();
        }
    }
}

// This variable must come first, so put it in a special section that is guaranteed to be put somewhere before `.percpu`.
#[used]
#[unsafe(link_section = ".percpu.init")]
pub(crate) static CPU_DATA: PerCpuData<CpuData> = PerCpuData::new(CpuData {
    this: &raw const LD_PERCPU_START as *mut CpuData,
    id: 0,
    kernel_stack: VirtAddr(0),
    user_stack: VirtAddr(0),
    online: false,
    present: false,
    scheduler: Scheduler::new(),
});

/// For regular per-CPU variables.
#[macro_export]
macro_rules! per_cpu {
    ($name:ident, $ty:ty, $value:expr) => {
        #[unsafe(link_section = ".percpu")]
        pub(crate) static $name: $crate::generic::cpu::PerCpuData<$ty> =
            $crate::generic::cpu::PerCpuData::new($value);
    };
}

/// Prepares per-CPU data for the boot CPU.
#[deny(dead_code)]
pub(crate) fn setup_bsp() {
    unsafe { arch::irq::interrupt_disable() };
    arch::cpu::setup_bsp();
}

/// Sets up all CPUs.
#[deny(dead_code)]
pub(crate) fn setup_all() {
    // Setup the BSP.
    arch::cpu::setup(CpuData::get());
    // TODO: Set up the rest.
}

/// Counts how many CPUs have been allocated. ID 0 is always used for the BSP.
static NUM_CPUS: AtomicUsize = AtomicUsize::new(1);

/// Extends the per-CPU data for a new CPU.
/// Returns the new CpuData context and the new CPU ID.
pub(crate) fn allocate_cpu() -> (&'static mut CpuData, usize) {
    let id = NUM_CPUS.fetch_add(1, Ordering::Relaxed);
    let percpu_size = (&raw const LD_PERCPU_END as usize - &raw const LD_PERCPU_START as usize);
    // TODO: Map new memory region, starting at end of the region. Needs page allocator.
    // TODO: Copy over default values.
    todo!();
}
