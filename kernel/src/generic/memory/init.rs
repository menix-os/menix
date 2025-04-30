//! Early memory setup and allocator.

use super::{
    PhysAddr,
    page::{AllocFlags, Page, PageAllocator},
};
use crate::{
    arch::virt::PageTableEntry,
    generic::{boot::BootInfo, misc::align_up},
};
use alloc::alloc::AllocError;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Global array that spans all usable physical memory.
/// It contains important metadata about a certain page.
/// This is virtually continuous, but not completely mapped in.
static PAGE_METADATA: &[Page] = &[];

// Symbols defined in the linker script so we can map ourselves in our page table.
unsafe extern "C" {
    unsafe static LD_KERNEL_START: u8;
    unsafe static LD_KERNEL_END: u8;
    unsafe static LD_TEXT_START: u8;
    unsafe static LD_TEXT_END: u8;
    unsafe static LD_RODATA_START: u8;
    unsafe static LD_RODATA_END: u8;
    unsafe static LD_DATA_START: u8;
    unsafe static LD_DATA_END: u8;
}

/// Points to the current page number for the next allocation.
static BUMP_PN: AtomicUsize = AtomicUsize::new(0);

struct BumpAllocator;
impl PageAllocator for BumpAllocator {
    fn alloc(pages: usize, flags: AllocFlags) -> Result<PhysAddr, AllocError> {
        let mem = BUMP_PN.fetch_add(pages, Ordering::Relaxed);
        return Ok(PhysAddr(mem * PageTableEntry::get_page_size()));
    }

    unsafe fn dealloc(addr: PhysAddr, pages: usize) {
        unreachable!()
    }
}

/// Bootstraps the memory allocators and kernel virtual page table.
///
/// # Safety
///
/// Must be called as soon as control is handed to [`crate::main`] or the system won't function!
#[deny(dead_code)]
pub fn init() {
    let info = BootInfo::get();
    let mut memory_map = info.memory_map.lock();

    // This function bootstraps the kernel allocators. It works as follows:
    // - Find how many pages we need to map the kernel and HHDM in a fresh page table.
    // - Find how much physical memory we need to populate the PAGE_METADATA array with backing memory.
    // - Find how much virtual memory we need to map the PAGE_METADATA array.
    // - Reserve all those pages in a memory region which can fit the total amount of pages.
    // - Map the kernel into a newly allocated page table with a a bump allocator.

    let pages = memory_map
        .iter()
        .map(|x| (x.address().inner() + x.length()) / PageTableEntry::get_page_size())
        .max()
        .unwrap_or(0);

    // The amount of pages required to map the kernel.
    let num_kernel = align_up(
        &raw const LD_KERNEL_END as usize - &raw const LD_KERNEL_START as usize,
        PageTableEntry::get_page_size(),
    ) / PageTableEntry::get_page_size();

    // The amount of pages required to map the HHDM.
    // We allocate 1GiB pages and map from address 0 to the highest address.
    let page_size = 1usize << (PageTableEntry::get_page_bits() + PageTableEntry::get_level_bits());
    let num_hhdm = align_up(pages, page_size) / page_size;

    // The amount of pages we need to keep track of the metadata.
    let num_meta_virt = pages * size_of::<Page>();
    // The amount of pages we need for the backing memory.
    let num_meta_phys = todo!();

    // Get the HHDM so we can write to physical memory.
    let hhdm_address = info
        .hhdm_address
        .expect("Can't bootstrap allocators without access to physical memory!");

    // Total amount of pages we have to allocate.
    let total_pages = todo!();

    // Find a region that can fit all this data.
    let bump_start = memory_map
        .iter()
        .find(|x| x.length() >= (total_pages)).expect("There is no memory region large enough bootstrap the data structures! We have way too little memory!");
}
