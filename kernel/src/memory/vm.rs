// Virtual memory management

pub use crate::arch::VirtManager;
use crate::{
    arch::{PageMap, PhysAddr, VirtAddr},
    boot::BootInfo,
    system::error::Errno,
};
use bitflags::bitflags;

bitflags! {
    /// POSIX page flags.
    pub struct ProtFlags: usize {
        const None = 0x00;
        const Read = 0x01;
        const Write = 0x02;
        const Exec = 0x04;
    }
}

/// Virtual memory manager.
/// Implementations must be called `VirtManager`.
pub trait CommonVirtManager {
    /// Initializes the virtual memory manager.
    /// This function should recreate the virtual mappings of the
    /// kernel executable and switch to the kernel-owned page map.
    unsafe fn init(info: &BootInfo);

    /// Sets a page map to be the current one.
    unsafe fn set_page_map(page_map: &PageMap);

    /// Maps a virtual address to physical memory.
    fn map_page(
        page_map: &PageMap,
        phys_addr: PhysAddr,
        virt_addr: VirtAddr,
        flags: ProtFlags,
    ) -> Result<(), Errno>;

    /// Modifies an existing mapping.
    fn remap_page(page_map: &PageMap, virt_addr: VirtAddr, flags: ProtFlags) -> Result<(), Errno>;

    /// Destroys an existing mapping.
    fn unmap_page(page_map: &PageMap, virt_addr: VirtAddr) -> Result<(), Errno>;

    /// Translates a mapped address into a physical address.
    fn virt_to_phys(addr: VirtAddr) -> Result<PhysAddr, Errno>;
}
