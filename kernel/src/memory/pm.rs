use crate::arch::PhysAddr;

/// Physical memory allocator.
/// Implementations must be called `PhysManager`.
pub trait CommonPhysManager {
    /// Allocates a given amount of pages.
    unsafe fn alloc(num_pages: usize) -> PhysAddr;

    /// Frees a region of pages allocated by `alloc()`.
    unsafe fn free(addr: PhysAddr, num_pages: usize);

    /// Returns a base address which linearly maps a physical address.
    unsafe fn get_phys_base() -> *mut u8;
}

#[derive(Clone, Debug, Default)]
pub enum PhysMemoryUsage {
    /// Free and usable memory.
    Free,
    /// Memory reserved by the System.
    Reserved,
    /// Used by boot loader structures.
    Bootloader,
    /// Kernel and modules are loaded here.
    Kernel,
    /// Unknown memory region.
    #[default]
    Unknown,
}

/// Describes a region of physical memory and what it's used for.
#[derive(Clone, Debug, Default)]
pub struct PhysMemory {
    /// Start address of the memory region.
    address: PhysAddr,
    /// Length of the memory region in bytes.
    length: usize,
    /// How this memory region is used.
    usage: PhysMemoryUsage,
}
