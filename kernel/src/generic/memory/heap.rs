use super::{
    PhysAddr, VirtAddr,
    phys::{self, RegionType},
};
use crate::{arch::virt::PageTableEntry, generic::misc};
use core::{
    alloc::{GlobalAlloc, Layout},
    num::NonZero,
};

pub struct HeapAllocator;

#[global_allocator]
pub static ALLOCATOR: HeapAllocator = HeapAllocator;

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // TODO: Use real allocation.
        return phys::alloc_bytes(
            NonZero::new(misc::align_up(
                layout.size(),
                PageTableEntry::get_page_size(),
            ))
            .unwrap(),
            RegionType::Kernel,
        )
        .expect("Out of memory!")
        .as_hhdm();
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
