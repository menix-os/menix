use super::{
    PhysAddr, VirtAddr,
    buddy::{Order, PageNumber},
    virt::VmLevel,
};
use crate::{
    arch::{self, virt::TrapFrame},
    generic::util::align_up,
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
static_assert!(0x1000 % size_of::<Page>() == 0);

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
        let pages = align_up(bytes, arch::virt::get_page_size(VmLevel::L1))
            / arch::virt::get_page_size(VmLevel::L1);
        return Self::alloc(pages, flags);
    }

    /// Deallocates a region of `pages` amount of consecutive pages.
    ///
    /// # Safety
    ///
    /// Deallocating arbitrary physical addresses is inherently unsafe, since it can cause the kernel to corrupt.
    unsafe fn dealloc(addr: PhysAddr, pages: usize);
}

/// Abstract information about a page fault.
pub struct PageFaultInfo {
    /// Fault caused by the user.
    pub caused_by_user: bool,
    /// The instruction pointer address.
    pub ip: VirtAddr,
    /// The address that was attempted to access.
    pub addr: VirtAddr,
    /// The cause of this page fault.
    pub kind: PageFaultKind,
}

/// The origin of the page fault.
pub enum PageFaultKind {
    /// Issue unclear (possible corruption).
    Unknown,
    /// Page is not mapped in the current page table.
    NotMapped,
    /// Page is mapped, but can't be read from.
    IllegalRead,
    /// Page is mapped, but can't be written to.
    IllegalWrite,
    /// Page is mapped, but can't be executed on.
    IllegalExecute,
}

/// Generic page fault handler. May reschedule and return a different context.
pub fn page_fault_handler<'a>(context: &'a TrapFrame, info: &PageFaultInfo) {
    if info.caused_by_user {
        // TODO: Send SIGSEGV and reschedule.
    }

    panic!(
        "Kernel caused an unrecoverable page fault: {}! IP: {:#x}, Address: {:#x}",
        match info.kind {
            PageFaultKind::Unknown => "Unknown cause",
            PageFaultKind::NotMapped => "Page was not mapped",
            PageFaultKind::IllegalRead => "Page can't be read from",
            PageFaultKind::IllegalWrite => "Page can't be written to",
            PageFaultKind::IllegalExecute => "Page can't be executed on",
        },
        info.ip.0,
        info.addr.0
    );
}
