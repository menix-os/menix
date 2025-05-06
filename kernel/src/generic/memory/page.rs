use core::sync::atomic::AtomicUsize;

use super::{
    PhysAddr, VirtAddr,
    buddy::{Order, PageNumber},
    virt::VmLevel,
};
use crate::{
    arch::{self, sched::Context},
    generic::util::{align_up, mutex::Mutex},
};
use alloc::alloc::AllocError;
use bitflags::bitflags;

/// Metadata about a physical page.
/// Keep this structure as small as possible, every single physical page has one!
#[derive(Debug)]
pub struct Page {
    pub prev: PageNumber,
    pub next: PageNumber,
    pub order: Order,
    _pad: u64,
}
static_assert!(size_of::<Page>() <= 48);
static_assert!(0x1000 % size_of::<Page>() == 0);

/// Global array that spans all usable physical memory.
/// It contains important metadata about a certain page.
/// This is virtually continuous, but not completely mapped in.
pub static PAGE_ARRAY: Mutex<&mut [Page]> = Mutex::new(&mut []);
pub static PAGE_ARRAY_ADDR: AtomicUsize = AtomicUsize::new(0);

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
        let pages = align_up(bytes, arch::memory::get_page_size(VmLevel::L1))
            / arch::memory::get_page_size(VmLevel::L1);
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
    pub cause: PageFaultCause,
}

bitflags! {
    /// The origin of the page fault.
    #[derive(Debug)]
    pub struct PageFaultCause: usize {
        /// If set, the fault occured in a mapped page.
        const Present = 1 << 0;
        /// If set, the fault was caused by a write.
        const Write = 1 << 1;
        /// If set, the fault was caused by an instruction fetch.
        const Fetch = 1 << 2;
        /// If set, the fault was caused by a user access.
        const User = 1 << 3;
    }
}

/// Generic page fault handler. May reschedule and return a different context.
pub fn page_fault_handler<'a>(context: &mut Context, info: &PageFaultInfo) {
    if info.caused_by_user {
        // TODO: Send SIGSEGV and reschedule.
        // Kill process.
        // Force immediate reschedule.
    }

    panic!(
        "Kernel caused an unrecoverable page fault: {:?}! IP: {:#x}, Address: {:#x}",
        info.cause, info.ip.0, info.addr.0
    );
}
