use crate::{boot::BootInfo, thread::thread::Thread};
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

// Re-export architecture specific types to make sure they exist in the implementation.
pub use internal::Arch;
pub use internal::Context;
pub use internal::Cpu;
pub use internal::PageMap;
pub use internal::PhysAddr;
pub use internal::VirtAddr;
pub use internal::VirtManager;

/// Common functionality that an architecture must provide.
/// Implementations must be called `Arch`.
pub trait CommonArch {
    /// Initializes basic I/O and starts the memory allocator.
    /// This function must be called as soon as `info` contains the memory map.
    fn early_init(info: &mut BootInfo);

    /// Prepares all available processors.
    /// Called before the scheduler is started.
    fn init(info: &mut BootInfo);

    /// Initializes a single core.
    fn init_cpu(cpu: &Cpu, boot_cpu: &Cpu);

    /// Gets the processor info of the executing processor.
    /// This information is processor-local and not shared with any other cores.
    fn current_cpu() -> &'static mut Cpu;
}

/// Common functionality for a processor state (aka context).
pub trait CommonContext {}

/// Common functionality for a virtual page map.
pub trait CommonPageMap {
    /// Creates a new page map.
    fn new() -> Self;

    /// Forks a page map from an existing one.
    fn fork(source: &Self) -> Self;
}

/// Common functionality of processor-local data.
/// Implementations must be called `Cpu`.
pub trait CommonCpu {
    /// Gets the ID of this processor.
    fn id(&self) -> usize;

    /// Gets the currently running thread.
    fn thread(&self) -> Option<&Arc<Thread>>;

    /// Sets the currently running thread.
    fn set_thread(&mut self, thread: &Arc<Thread>);
}
