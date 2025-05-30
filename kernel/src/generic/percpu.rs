//! Per-CPU data structures.

use super::{memory::VirtAddr, sched::Scheduler};
use crate::arch;
use core::sync::atomic::{AtomicUsize, Ordering};

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
    /// A scheduler instance on this CPU.
    pub scheduler: Scheduler,
}

impl CpuData {
    /// Gets the data for the current CPU.
    pub fn get() -> &'static mut CpuData {
        return unsafe { arch::core::get_per_cpu().as_mut().unwrap() };
    }

    /// Gets the data for a specific CPU.
    /// # Safety
    /// The caller must make sure that `id` is valid.
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

    /// Gets the inner CPU-local instance of this field.
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
pub static CPU_DATA: PerCpuData<CpuData> = PerCpuData::new(CpuData {
    this: &raw const LD_PERCPU_START as *mut CpuData,
    id: 0,
    kernel_stack: VirtAddr::null(),
    user_stack: VirtAddr::null(),
    online: false,
    present: false,
    scheduler: Scheduler::uninit(),
});

/// Counts how many CPUs have been allocated. ID 0 is always used for the BSP.
static NUM_CPUS: AtomicUsize = AtomicUsize::new(1);

/// Extends the per-CPU data for a new CPU.
/// Returns the new CpuData context and the new CPU ID.
pub(crate) fn allocate_cpu() -> (&'static mut CpuData, usize) {
    let _id = NUM_CPUS.fetch_add(1, Ordering::Relaxed);
    let _percpu_size = &raw const LD_PERCPU_END as usize - &raw const LD_PERCPU_START as usize;
    // TODO: Map new memory region, starting at end of the region. Needs page allocator.
    // TODO: Copy over default values.
    todo!();
}
