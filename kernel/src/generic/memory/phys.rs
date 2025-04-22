//! Physical page allocation.
//! This allocator uses the buddy allocation algorithm.
//! It's very loosely inspired by Maestro's

use super::{PhysAddr, VirtAddr};
use crate::{
    arch::virt::PageTableEntry,
    generic::{self, boot::PhysMemoryUsage},
};
use alloc::{alloc::AllocError, slice};
use core::ptr::{NonNull, null_mut};
use spin::Mutex;

/// Allocates `pages` amount of consecutive pages.
pub fn alloc_pages(pages: usize, range: RangeType) -> Result<PhysAddr, AllocError> {
    todo!();
}

/// Deallocates a region of `pages` amount of consecutive pages.
///
/// # Safety
///
/// Deallocating arbitrary physical addresses is inherently unsafe, since it can cause the kernel to crash.
pub unsafe fn dealloc_pages(addr: PhysAddr, pages: usize) {
    todo!();
}

pub type PageNumber = u32;
pub type Order = u8;

const MAX_ORDER: Order = 17;

#[repr(u32)]
pub enum RangeType {
    /// Memory suitable for any use case.
    Kernel = 1 << 0,
    /// Memory suitable for
    User = 1 << 1,
    /// Any zone.
    Any = Self::Kernel as u32 | Self::User as u32,
}

// TODO: Use IRQ disabling mutex instead.
unsafe impl Send for Region {}
static REGIONS: Mutex<[Option<Region>; 128]> = Mutex::new([None; 128]);

/// A region of physical memory.
#[derive(Clone, Copy)]
pub(crate) struct Region {
    /// Start of the metadata block for this region.
    meta: *mut Page,
    /// Start of this memory region.
    phys: PhysAddr,
    /// Amount of pages available in total.
    num_pages: PageNumber,
    /// Amount of pages currently in use.
    num_used_pages: PageNumber,
    lists: [Option<NonNull<Page>>; (MAX_ORDER + 1) as usize],
}

impl Region {
    pub fn new(meta: VirtAddr, phys: PhysAddr, num_pages: PageNumber) -> Self {
        // Amount of pages which are going to get consumed by the metadata.
        let meta_page_size = generic::misc::align_up(
            num_pages as usize * size_of::<Page>(),
            PageTableEntry::get_page_size(),
        ) / PageTableEntry::get_page_size();

        Self {
            meta: meta.as_ptr(),
            phys,
            num_pages: num_pages - meta_page_size as PageNumber,
            num_used_pages: 0,
            lists: [None; (MAX_ORDER + 1) as usize],
        }
    }

    /// Registers a region of usable memory for the page allocator.
    pub fn register(self) {
        let mut regions = REGIONS.lock();
        for region in regions.iter_mut() {
            match region {
                Some(_) => continue,
                None => {
                    print!(
                        "memory: Registered {:#018x}, {} pages\n",
                        self.phys.0, self.num_pages
                    );
                    *region = Some(self);
                    return;
                }
            }
        }
    }

    /// Returns the size of this region in pages.
    #[inline]
    pub fn get_size(&self) -> usize {
        self.num_pages as usize
    }

    #[inline]
    fn pages(&self) -> &'static mut [Page] {
        unsafe { slice::from_raw_parts_mut(self.meta, self.num_pages as usize) }
    }
}

/// Used to represent a used page.
const PAGE_USED: PageNumber = PageNumber::MAX;

/// Metadata about a physical page.
/// Keep this structure as small as possible, every single physical page has one!
#[repr(packed)]
#[derive(Debug)]
pub struct Page {
    prev: PageNumber,
    next: PageNumber,
    order: Order,
}
static_assert!(size_of::<Page>() <= 48);

impl Page {
    /// Gets the page number of this page relative to the given region.
    fn id(&self, region: &Region) -> PageNumber {
        let self_off = self as *const _ as usize;
        let meta = region.meta as *const _ as usize;
        debug_assert!(self_off >= meta);

        return ((self_off - meta) / size_of::<Self>()) as PageNumber;
    }

    /// Gets the page number of the buddy frame inside a region.
    #[inline]
    fn buddy_id(&self, region: &Region) -> PageNumber {
        self.id(region) ^ (1 << self.order) as PageNumber
    }

    /// Links this page to a region.
    fn link(&mut self, region: &mut Region) {
        let id = self.id(region);
        self.prev = id;
        self.next = if let Some(mut next) = region.lists[self.order as usize] {
            let next = unsafe { next.as_mut() };
            debug_assert!(!next.is_used());
            next.prev = id;
            next.id(region)
        } else {
            id
        };
        region.lists[self.order as usize] = NonNull::new(self);
    }

    /// Unlinks this page from a region.
    fn unlink(&mut self, region: &mut Region) {}

    /// Checks if this page is used.
    #[inline]
    fn is_used(&self) -> bool {
        (self.prev == PAGE_USED) || (self.next == PAGE_USED)
    }

    /// Marks this page as used.
    #[inline]
    fn mark_used(&mut self) {
        self.prev = PAGE_USED;
        self.next = PAGE_USED;
    }

    /// Marks this page as free.
    #[inline]
    fn mark_free(&mut self, region: &Region) {
        let id = self.id(region);
        self.prev = id;
        self.next = id;
    }

    /// Checks if the page state is valid.
    #[cfg(debug_assertions)]
    fn validate(&self, zone: &Region) {
        debug_assert!(self.prev == PAGE_USED || self.prev < zone.num_pages);
        debug_assert!(self.next == PAGE_USED || self.next < zone.num_pages);
        debug_assert!(self.order <= MAX_ORDER);
    }
}
