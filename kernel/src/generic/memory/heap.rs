use super::{
    PhysAddr, VirtAddr,
    phys::{self, RangeType},
};
use crate::{arch::virt::PageTableEntry, generic::misc};
use core::alloc::{GlobalAlloc, Layout};

pub struct HeapAllocator;

#[global_allocator]
pub static ALLOCATOR: HeapAllocator = HeapAllocator;

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // TODO: Use real allocation.
        let phys = phys::alloc_pages(
            misc::align_up(layout.size(), PageTableEntry::get_page_size())
                / PageTableEntry::get_page_size(),
            RangeType::Kernel,
        )
        .expect("Out of memory!");

        return phys.as_hhdm();
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // TODO: Use real deallocation.
        unsafe {
            phys::dealloc_pages(
                VirtAddr::from(ptr).as_hhdm().unwrap(),
                misc::align_up(layout.size(), PageTableEntry::get_page_size())
                    / PageTableEntry::get_page_size(),
            );
        }
    }
}
