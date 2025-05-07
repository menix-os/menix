use super::internal;
use crate::generic::memory::PhysAddr;
use crate::generic::memory::VirtAddr;
use crate::generic::memory::virt::VmLevel;

pub use internal::virt::PageTableEntry;

/// Gets the page size for a given level.
pub fn get_page_size(level: VmLevel) -> usize {
    1 << (get_page_bits() + (get_level_bits() * level as usize))
}

/// Gets the amount of bits in a page.
pub fn get_page_bits() -> usize {
    internal::virt::get_page_bits()
}

/// Gets the amount of bits contained in a single page level.
pub fn get_level_bits() -> usize {
    internal::virt::get_level_bits()
}

/// Gets the maximum amount of pages supported.
pub fn get_max_level() -> VmLevel {
    internal::virt::get_max_level()
}

/// Sets a given page table as the active one on this CPU.
///
/// # Safety
///
/// The caller must make sure that all kernel pages are still mapped as they were before.
pub unsafe fn set_page_table(phys: PhysAddr) {
    unsafe { internal::virt::set_page_table(phys) };
}

/// Invalidates a TLB entry cache.
pub fn flush_tlb(addr: VirtAddr) {
    internal::virt::flush_tlb(addr);
}
