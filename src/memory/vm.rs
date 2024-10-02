// Virtual memory management

use crate::boot::BootInfo;

/// Virtual memory manager.
/// Implementations must be called `VirtManager`.
pub trait CommonVirtManager {
    unsafe fn init(info: &BootInfo);
}
