use crate::boot::BootInfo;
use crate::generic::schedule;
use crate::generic::schedule::PreemptionGuard;
use alloc::boxed::Box;
use alloc::sync::Arc;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
mod internal {
    pub use super::x86_64::*;
}

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "aarch64")]
mod internal {
    pub use super::aarch64::*;
}

#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "riscv64")]
mod internal {
    pub use super::riscv64::*;
}

#[cfg(target_arch = "loongarch64")]
pub mod loongarch64;
#[cfg(target_arch = "loongarch64")]
mod internal {
    pub use super::loongarch64::*;
}

// Re-export architecture specific types and functions to make sure they exist.
pub use internal::current_cpu;
pub use internal::early_init;
pub use internal::get_page_size;
pub use internal::init;
pub use internal::init_cpu;
pub use internal::scheduler::Context;
pub use internal::virt::PageMap;

/// Represents a physical address. It can't be directly read from or written to.
pub use internal::PhysAddr;
/// Represents a virtual address. It can't be directly read from or written to.
/// Note: Not the same as a pointer. A `VirtAddr` might point into another
/// process's memory that is not mapped in the kernel.
pub use internal::VirtAddr;

// TODO
pub struct Thread {}

/// Processor-local information.
#[repr(C, align(0x10))]
pub struct PerCpu {
    /// Unique identifier of this CPU.
    pub id: usize,
    /// Stack pointer for kernel mode.
    pub kernel_stack: VirtAddr,
    /// Stack pointer for user mode.
    pub user_stack: VirtAddr,
    /// Current thread running on this CPU.
    pub thread: Option<Arc<Thread>>,
    /// Amount of ticks the current thread has been running for.
    pub ticks_active: usize,
    /// Whether this CPU is enabled.
    pub enabled: bool,
}
