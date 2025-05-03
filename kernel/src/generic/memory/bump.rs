//! Early memory setup and allocator.

use super::{
    PhysAddr,
    page::{AllocFlags, PageAllocator},
    virt::VmLevel,
};
use crate::arch::{self};
use alloc::alloc::AllocError;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Points to the current address for the next allocation.
pub(crate) static BUMP_CURRENT: AtomicUsize = AtomicUsize::new(0);

pub(crate) struct BumpAllocator;
impl PageAllocator for BumpAllocator {
    fn alloc(pages: usize, flags: AllocFlags) -> Result<PhysAddr, AllocError> {
        let bytes = pages * arch::memory::get_page_size(VmLevel::L1);
        let mem = PhysAddr(BUMP_CURRENT.fetch_add(bytes, Ordering::Relaxed));

        if flags.contains(AllocFlags::Zeroed) {
            unsafe { (mem.as_hhdm() as *mut u8).write_bytes(0, bytes) };
        }

        return Ok(mem);
    }

    unsafe fn dealloc(_addr: PhysAddr, _pages: usize) {
        unimplemented!(
            "The bump allocator is not supposed to free anything. Remove this .dealloc()"
        )
    }
}
