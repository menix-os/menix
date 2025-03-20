use crate::{
    arch::{PhysAddr, VirtAddr, virt::PageTableEntry},
    generic::misc::align_up,
};
use core::ptr::write_bytes;
use spin::Mutex;
use talc::{ClaimOnOom, Span, Talc, Talck};

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

pub static EARLY_MEMORY: [u8; 0x10000] = [0u8; 0x10000];

#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> = Talc::new(unsafe {
    ClaimOnOom::new(Span::from_array(
        core::ptr::addr_of!(EARLY_MEMORY).cast_mut(),
    ))
})
.lock();

/// Initializes the physical memory manager.
/// `temp_base`: A temporary base address which can be used to directly access physical memory.
pub fn init(memory_map: &mut [PhysMemory], temp_base: VirtAddr) {
    let mut alloc = ALLOCATOR.lock();
    for region in memory_map {
        if region.usage == PhysMemoryUsage::Free {
            print!("memory: Claiming memory region {:?}\n", region);
            unsafe {
                alloc.claim(Span::from_base_size(
                    (region.address + temp_base) as *mut u8,
                    region.length,
                ));
            }
            print!("memory: Claimed region {:?}\n", region);
        }
    }
    print!("memory: Initialized memory allocator.\n");
}
