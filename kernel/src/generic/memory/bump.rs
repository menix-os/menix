//! Early memory setup and allocator.

use super::{
    PhysAddr,
    page::{AllocFlags, PageAllocator},
    virt::VmLevel,
};
use crate::arch::{self};
use alloc::alloc::AllocError;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Points to the current page number for the next allocation.
pub(crate) static BUMP_PN: AtomicUsize = AtomicUsize::new(0);

pub(crate) struct BumpAllocator;
impl PageAllocator for BumpAllocator {
    fn alloc(pages: usize, flags: AllocFlags) -> Result<PhysAddr, AllocError> {
        let bytes = pages * arch::virt::get_page_size(VmLevel::L1);
        let mem = PhysAddr(BUMP_PN.fetch_add(bytes, Ordering::Relaxed));
        unsafe { (mem.as_hhdm() as *mut u8).write_bytes(0, bytes) };

        return Ok(mem);
    }

    unsafe fn dealloc(addr: PhysAddr, pages: usize) {
        unimplemented!(
            "The bump allocator is not supposed to free anything. Remove this .dealloc()"
        )
    }
}
