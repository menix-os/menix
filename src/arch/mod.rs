use crate::{boot::BootInfo, thread::thread::Thread};
use alloc::sync::Arc;

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
mod internal {
    pub use super::x86_64::pm::*;
    pub use super::x86_64::vm::*;
    pub use super::x86_64::*;
}

// Re-export architecture specific types to make sure they exist in the implementation.
pub use internal::Arch;
pub use internal::Context;
pub use internal::Cpu;
pub use internal::PhysAddr;
pub use internal::PhysManager as pm;
pub use internal::VirtAddr;
pub use internal::VirtManager as vm;

/// Common functionality that an architecture must provide.
/// Before all cores are properly initialized, every operation is unsafe.
/// Implementations must be called `Arch`.
pub trait CommonArch {
    /// Initializes basic I/O and starts the memory allocator.
    /// Called before anything else during boot.
    unsafe fn early_init(info: &BootInfo);

    /// Prepares all available processors.
    /// Called before the scheduler is started.
    unsafe fn init(info: &BootInfo);

    /// Initializes a single core.
    unsafe fn init_cpu(cpu: &Cpu, boot_cpu: &Cpu);

    /// Gets the processor info of the executing processor.
    fn current_cpu() -> &'static mut Cpu;
}

/// Common functionality of processor-local data.
/// Implementations must be called `Cpu`.
pub trait CommonCpu {
    /// Gets the ID of this processor.
    fn id(&self) -> usize;

    /// Gets the currently running thread.
    fn thread(&self) -> &Option<Arc<Thread>>;

    /// Sets the currently running thread.
    fn set_thread(&mut self, thread: &Arc<Thread>);
}
