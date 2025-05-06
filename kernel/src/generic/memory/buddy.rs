//! Physical page allocation.
//! This allocator uses the buddy algorithm.

use super::{
    PhysAddr,
    page::{Page, PageAllocator},
};
use crate::{
    arch::{self},
    generic::{
        self,
        memory::{
            page::{AllocFlags, PAGE_ARRAY, PAGE_ARRAY_ADDR},
            virt::VmLevel,
        },
        util::mutex::IrqMutex,
    },
};
use alloc::alloc::AllocError;
use core::{
    hint::likely,
    ptr::{NonNull, write_bytes},
    sync::atomic::Ordering,
};

/// Allocates `bytes` amount in bytes of consecutive pages.
pub struct BuddyAllocator;
impl PageAllocator for BuddyAllocator {
    fn alloc(pages: usize, flags: AllocFlags) -> Result<PhysAddr, AllocError> {
        if pages == 0 {
            return Err(AllocError);
        }

        let block_order = get_order(pages);
        let result = alloc(block_order, flags);

        return result;
    }

    unsafe fn dealloc(_addr: PhysAddr, _pages: usize) {}
}

fn alloc(order: Order, flags: AllocFlags) -> Result<PhysAddr, AllocError> {
    if order > MAX_ORDER {
        return Err(AllocError);
    }

    let mut regions = REGIONS.lock();

    // Determine what the highest allowed address for this allocation is.
    let search_limit = if flags.contains(AllocFlags::Kernel32) {
        PhysAddr(u32::MAX as usize)
    } else {
        PhysAddr(usize::MAX)
    };

    let (mut frame, region) = regions
        .iter_mut()
        .filter_map(|reg| match reg {
            Some(r) => {
                if r.get_end() <= search_limit {
                    Some((r.next_free_page(order)?, r))
                } else {
                    None
                }
            }
            None => None,
        })
        .next()
        .ok_or(AllocError)?;
    let frame = unsafe { frame.as_mut() };

    debug_assert!(!frame.is_used());

    frame.split(region, order);
    let addr = frame.addr();

    debug_assert!(addr >= region.phys && addr.0 < region.phys.0 + region.get_size());

    frame.mark_used();
    region.num_used_pages += 1 << order;

    // If required, zero new memory.
    if flags.contains(AllocFlags::Zeroed) {
        unsafe {
            write_bytes(
                addr.as_hhdm::<u8>(),
                0,
                arch::memory::get_page_size(VmLevel::L1) << frame.order,
            );
        }
    }

    Ok(addr)
}

/// Deallocates physical memory given out by [`alloc`].
/// # Safety
/// The caller has to verify that `addr` is an address given out by [`alloc`].
/// The same allocation must also never be freed more than once, as this could otherwise corrupt global state.
unsafe fn dealloc(addr: PhysAddr, order: Order) {
    todo!();
}

pub type PageNumber = u64;
pub type Order = u8;

#[inline]
pub const fn get_order(pages: usize) -> Order {
    if likely(pages != 0) {
        (usize::BITS - pages.leading_zeros()) as _
    } else {
        0
    }
}

const MAX_ORDER: Order = 20;

static REGIONS: IrqMutex<[Option<Region>; 128]> = IrqMutex::new([None; 128]);

/// A region of physical memory.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Region {
    /// Start of this memory region.
    phys: PhysAddr,
    /// Amount of pages available in total.
    num_pages: PageNumber,
    /// Amount of pages currently in use.
    num_used_pages: PageNumber,
    lists: [Option<NonNull<Page>>; (MAX_ORDER + 1) as usize],
}

impl Region {
    pub fn new(phys: PhysAddr, num_pages: PageNumber) -> Self {
        // Amount of pages which are going to get consumed by the metadata.
        let meta_size = generic::util::align_up(
            num_pages as usize * size_of::<Page>(),
            arch::memory::get_page_size(VmLevel::L1),
        );
        let meta_page_size = meta_size / arch::memory::get_page_size(VmLevel::L1);

        let mut result = Self {
            phys: PhysAddr(phys.0 + meta_size),
            num_pages: num_pages - meta_page_size as PageNumber,
            num_used_pages: 0,
            lists: [None; (MAX_ORDER + 1) as usize],
        };

        let mut frames = PAGE_ARRAY.lock();
        let mut frame: PageNumber = 0;
        let mut order = MAX_ORDER;
        while frame < result.num_pages as PageNumber {
            let p = (1 as PageNumber) << (order as PageNumber);
            if frame + p > result.num_pages {
                order -= 1;
                continue;
            }
            let f = frames.get_mut(frame as usize).unwrap();
            f.mark_free();
            f.order = order;
            f.link(&mut result);
            frame += p;
        }

        return result;
    }

    /// Registers a region of usable memory for the page allocator.
    pub fn register(self) {
        let mut regions = REGIONS.lock();
        for region in regions.iter_mut() {
            match region {
                Some(_) => continue,
                None => {
                    *region = Some(self);
                    return;
                }
            }
        }
    }

    /// Returns the size of this region in bytes.
    #[inline]
    pub fn get_size(&self) -> usize {
        self.num_pages as usize * arch::memory::get_page_size(VmLevel::L1)
    }

    /// Returns the highest address covered by this region.
    #[inline]
    pub fn get_end(&self) -> PhysAddr {
        PhysAddr(self.phys.0 + (self.num_pages as usize * arch::memory::get_page_size(VmLevel::L1)))
    }

    fn next_free_page(&mut self, order: Order) -> Option<NonNull<Page>> {
        let mut page = self.lists[(order as usize)..]
            .iter_mut()
            .filter_map(|f| *f)
            .next()?;
        let p = unsafe { page.as_mut() };
        debug_assert!(p.order >= order);
        debug_assert!(!p.is_used());
        debug_assert!(p.addr() >= self.phys);
        debug_assert!(p.addr().0 < self.phys.0 + self.get_size());
        Some(page)
    }
}

/// Used to represent a used page.
const PAGE_USED: PageNumber = PageNumber::MAX;

impl Page {
    /// Gets the page number of this page relative to the given region.
    fn id(&self) -> PageNumber {
        let self_off = self as *const _ as usize;
        let meta = PAGE_ARRAY_ADDR.load(Ordering::Relaxed);
        debug_assert!(self_off >= meta);

        return ((self_off - meta) / size_of::<Self>()) as PageNumber;
    }

    /// Gets the page number of the buddy frame inside a region.
    #[inline]
    fn buddy_id(&self) -> PageNumber {
        self.id() ^ (1 << self.order) as PageNumber
    }

    /// Gets the physical address of the page.
    fn addr(&self) -> PhysAddr {
        PhysAddr(self.id() as usize * arch::memory::get_page_size(VmLevel::L1))
    }

    /// Links this page to a region.
    fn link(&mut self, region: &mut Region) {
        let id = self.id();
        self.prev = id;
        self.next = if let Some(mut next) = region.lists[self.order as usize] {
            let next = unsafe { next.as_mut() };
            debug_assert!(!next.is_used());
            next.prev = id;
            next.id()
        } else {
            id
        };
        region.lists[self.order as usize] = NonNull::new(self);
    }

    /// Unlinks this page from a region.
    fn unlink(&mut self, region: &mut Region) {
        let mut pages = PAGE_ARRAY.lock();
        let id = self.id();
        let has_prev = self.prev != id;
        let has_next = self.next != id;

        let first = &mut region.lists[self.order as usize];
        if first.map(NonNull::as_ptr) == Some(self) {
            *first = if has_next {
                NonNull::new(&mut pages[self.next as usize])
            } else {
                None
            };
        }

        // TODO
        if has_prev {
            pages[self.prev as usize].next = if has_next { self.next } else { self.prev };
        }
        if has_next {
            pages[self.next as usize].prev = if has_prev { self.prev } else { self.next };
        }
    }

    fn split(&mut self, region: &mut Region, order: Order) {
        debug_assert!(!self.is_used());
        debug_assert!(order <= MAX_ORDER);
        debug_assert!(self.order >= order);

        self.unlink(region);

        let mut pages = PAGE_ARRAY.lock();

        while self.order > order {
            self.order -= 1;
            let buddy = self.buddy_id();
            if buddy >= region.num_pages {
                break;
            }

            let buddy_frame = &mut pages[buddy as usize];
            buddy_frame.mark_free();
            buddy_frame.order = self.order;
            buddy_frame.link(region);
        }
    }

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
    fn mark_free(&mut self) {
        let id = self.id();
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
