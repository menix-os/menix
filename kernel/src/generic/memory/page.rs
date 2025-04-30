use crate::{arch::virt::PageTableEntry, generic::misc::align_up};

use super::{
    PhysAddr,
    buddy::{Order, PageNumber},
};
use alloc::alloc::AllocError;

/// Metadata about a physical page.
/// Keep this structure as small as possible, every single physical page has one!
#[derive(Debug)]
pub struct Page {
    pub prev: PageNumber,
    pub next: PageNumber,
    pub order: Order,
    _pad: u32,
}
static_assert!(size_of::<Page>() <= 48);
static_assert!(PageTableEntry::get_page_size() % size_of::<Page>() == 0);

bitflags::bitflags! {
    pub struct AllocFlags: usize {
        /// Only consider physical memory below 4GiB.
        const Kernel32 = 1 << 0;
        /// Allocated memory has to be initialized to zero.
        const Zeroed = 1 << 2;
    }
}

pub trait PageAllocator {
    /// Allocates `pages` amount of consecutive pages.
    fn alloc(pages: usize, flags: AllocFlags) -> Result<PhysAddr, AllocError>;

    /// Allocates enough consecutive pages to fit `bytes` amount of bytes.
    fn alloc_bytes(bytes: usize, flags: AllocFlags) -> Result<PhysAddr, AllocError> {
        let pages =
            align_up(bytes, PageTableEntry::get_page_size()) / PageTableEntry::get_page_size();
        return Self::alloc(pages, flags);
    }

    /// Deallocates a region of `pages` amount of consecutive pages.
    ///
    /// # Safety
    ///
    /// Deallocating arbitrary physical addresses is inherently unsafe, since it can cause the kernel to corrupt.
    unsafe fn dealloc(addr: PhysAddr, pages: usize);
}
