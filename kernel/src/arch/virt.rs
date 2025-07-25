use super::internal;
use crate::generic::memory::{PhysAddr, VirtAddr, virt::VmLevel};

pub use internal::virt::PageTableEntry;

/// Gets the page size for a given level.
pub fn get_page_size(level: VmLevel) -> usize {
    1 << (get_page_bits() + (get_level_bits() * level as usize))
}

/// Gets the highest possible shift for a canonical virtual address.
pub fn get_highest_bit_shift() -> usize {
    get_level_bits() * get_num_levels() + get_page_bits()
}

/// Gets the amount of bits in a page.
pub fn get_page_bits() -> usize {
    internal::virt::get_page_bits()
}

/// Gets the amount of bits contained in a single page level.
pub fn get_level_bits() -> usize {
    internal::virt::get_level_bits()
}

/// Gets the highest supported mappable page level.
/// This is different from [`get_num_levels`], because not all levels can be PTE leaves.
pub fn get_max_leaf_level() -> VmLevel {
    internal::virt::get_max_leaf_level()
}

/// Gets the amount of page levels in a virtual address.
pub fn get_num_levels() -> usize {
    internal::virt::get_num_levels()
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

// # Note
// This module is only used to ensure the API is correctly implemented,
// since associated functions are more complicated. Not to be used directly.
#[doc(hidden)]
#[allow(unused)]
mod api {
    use super::PageTableEntry;
    use crate::generic::memory::{PhysAddr, virt::VmFlags};

    /// Returns a PTE which represents an empty slot.
    const fn pte_empty() -> PageTableEntry {
        PageTableEntry::empty()
    }

    /// Returns a new PTE with a set address.
    const fn pte_new(address: PhysAddr, flags: VmFlags, level: usize) -> PageTableEntry {
        PageTableEntry::new(address, flags, level)
    }

    /// Returns the inner representation of the PTE.
    const fn pte_inner(pte: &PageTableEntry) -> usize {
        PageTableEntry::inner(pte)
    }

    /// Returns true if the PTE is present.
    fn pte_is_present(pte: &PageTableEntry) -> bool {
        pte.is_present()
    }

    /// Returns true if the PTE is a directory, aka not a leaf entry.
    fn pte_is_directory(pte: &PageTableEntry, level: usize) -> bool {
        pte.is_directory(level)
    }

    fn pte_is_dirty(pte: &PageTableEntry) -> bool {
        pte.is_dirty()
    }

    /// Returns the contained address pointed to by the PTE.
    fn pte_address(pte: &PageTableEntry) -> PhysAddr {
        pte.address()
    }
}
