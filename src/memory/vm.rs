// Virtual memory management

pub use crate::arch::VirtManager;
use crate::boot::BootInfo;

/// Virtual memory manager.
/// Implementations must be called `VirtManager`.
pub trait CommonVirtManager {
    /// Initializes the virtual memory manager.
    /// This function should recreate the virtual mappings of the
    /// kernel executable and switch to the kernel-owned page map.
    unsafe fn init(info: &BootInfo);
}
