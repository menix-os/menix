use crate::arch::page::PageTableEntry;
use alloc::alloc::{AllocError, Allocator};
use core::{
    alloc::Layout,
    ptr::{self, NonNull},
};
use spin::Mutex;
use talc::{ClaimOnOom, Span, Talc, Talck};

pub mod mmio;
pub mod page;
pub mod virt;

/// Represents a physical address. It can't be directly read from or written to.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub usize);

/// Represents a virtual address. It can't be directly read from or written to.
/// Note: Not the same as a pointer. A `VirtAddr` might point into another
/// process's memory that is not mapped in the kernel.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub usize);

impl<T> From<*const T> for VirtAddr {
    fn from(ptr: *const T) -> Self {
        Self(ptr as usize)
    }
}

impl<T> From<*mut T> for VirtAddr {
    fn from(ptr: *mut T) -> Self {
        Self(ptr as usize)
    }
}

impl<T> From<NonNull<T>> for VirtAddr {
    fn from(ptr: NonNull<T>) -> Self {
        Self(ptr.as_ptr() as usize)
    }
}

impl VirtAddr {
    pub fn as_ptr<T>(self) -> *mut T {
        return ptr::with_exposed_provenance_mut(self.0);
    }

    /// Returns the physical address mapped in the kernel for this [`VirtAddr`].
    pub fn get_kernel_phys(self) -> Option<PhysAddr> {
        return self
            .0
            .checked_sub(PageTableEntry::get_hhdm_addr().0)
            .map(PhysAddr);
    }
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
            address: PhysAddr(0),
            length: 0,
            usage: PhysMemoryUsage::Unknown,
        }
    }
}

pub static EARLY_MEMORY: [u8; 0x10000] = [0u8; 0x10000];

#[global_allocator]
pub static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> = Talc::new(unsafe {
    ClaimOnOom::new(Span::from_array(
        core::ptr::addr_of!(EARLY_MEMORY).cast_mut(),
    ))
})
.lock();

/// Allocates data that is aligned on page boundaries.
pub struct PageAlloc;
unsafe impl Allocator for PageAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        ALLOCATOR.allocate(
            layout
                .align_to(1 << PageTableEntry::get_page_bits())
                .unwrap(),
        )
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            ALLOCATOR.deallocate(
                ptr,
                layout
                    .align_to(1 << PageTableEntry::get_page_bits())
                    .unwrap(),
            )
        }
    }
}

/// Initializes the physical memory manager.
/// `temp_base`: A temporary base address which can be used to directly access physical memory.
#[deny(dead_code)]
pub(crate) fn init(memory_map: &[PhysMemory], temp_base: VirtAddr) {
    let mut alloc = ALLOCATOR.lock();
    for region in memory_map {
        if region.usage != PhysMemoryUsage::Free {
            continue;
        }

        let actual = unsafe {
            alloc.claim(Span::from_base_size(
                temp_base.as_ptr::<u8>().byte_add(region.address.0),
                region.length,
            ))
        };

        match actual {
            Ok(x) => {
                if let Some((start, end)) = x.get_base_acme() {
                    print!(
                        "memory: Claimed memory region [{:p} - {:p}] ({:#x} bytes)\n",
                        start,
                        end,
                        x.size()
                    );
                }
            }
            Err(_) => todo!(),
        }
    }
    print!("memory: Initialized memory allocator.\n");
}
