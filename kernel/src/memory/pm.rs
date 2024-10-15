pub use crate::arch::PhysManager;
use crate::{
    arch::{PhysAddr, PAGE_SIZE},
    boot::BootInfo,
};
use core::ptr::write_bytes;

/// Physical memory allocator.
/// Implementations must be called `PhysManager`.
pub trait CommonPhysManager {
    /// Initializes the memory manager.
    unsafe fn init(info: &mut BootInfo);

    /// Allocates a given amount of pages.
    fn alloc(num_pages: usize) -> PhysAddr;

    /// Allocates a given amount of zero-initialized pages.
    fn alloc_zeroed(num_pages: usize) -> PhysAddr {
        let addr = Self::alloc(num_pages);
        unsafe {
            write_bytes(addr as *mut u8, 0, PAGE_SIZE);
        };
        return addr;
    }

    /// Frees a region of pages allocated by `alloc()`.
    fn free(addr: PhysAddr, num_pages: usize);

    /// Returns a base address which linearly maps a physical address.
    fn phys_base() -> *mut u8;
}

/// Describes how a memory region is used.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
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

/// Describes a region of physical memory.
#[derive(Clone, Copy, Debug, Default)]
pub struct PhysMemory {
    /// Start address of the memory region.
    pub address: PhysAddr,
    /// Length of the memory region in bytes.
    pub length: usize,
    /// How this memory region is used.
    pub usage: PhysMemoryUsage,
}

impl PhysMemory {
    pub const fn new() -> Self {
        Self {
            address: 0,
            length: 0,
            usage: PhysMemoryUsage::Unknown,
        }
    }
}
