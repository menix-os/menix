use super::PhysAddr;
use crate::{
    arch,
    generic::{
        boot::PhysMemory,
        memory::virt::VmLevel,
        util::{align_up, mutex::Mutex},
    },
};
use alloc::alloc::AllocError;
use bitflags::bitflags;
use core::{
    ptr::{NonNull, null_mut, write_bytes},
    slice,
    sync::atomic::{AtomicPtr, Ordering},
};

bitflags! {
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
    /// # Safety
    /// Deallocating arbitrary physical addresses is inherently unsafe, since it can cause the kernel to corrupt.
    unsafe fn dealloc(addr: PhysAddr, pages: usize);
}

/// Metadata about a physical page.
/// Keep this structure as small as possible, every single physical page has one!
#[derive(Debug)]
#[repr(C)]
pub struct Page {
    pub next: Option<NonNull<Page>>,
    pub max_order: u8,
    pub current_order: u8,
    pub on_free_list: bool,
}
static_assert!(size_of::<Page>() <= 16);
static_assert!(0x1000 % size_of::<Page>() == 0);

pub static PAGE_DB: Mutex<&'static mut [Page]> = Mutex::new(&mut []);
pub static PAGE_DB_START: AtomicPtr<()> = AtomicPtr::new(null_mut());

impl Page {
    pub fn idx_from_addr(address: PhysAddr) -> usize {
        assert!(address.0 % arch::memory::get_page_size(VmLevel::L1) == 0);

        let pn = address.0 >> arch::memory::get_page_bits();
        return pn;
    }

    /// Returns the page number of this page.
    fn get_pn(&self) -> usize {
        let page: *const Page = self;
        let page = page as usize;
        let db = PAGE_DB_START.load(Ordering::Relaxed) as usize;
        assert!(page >= db, "{:#018x} >= {:#018x}", page, db);
        return (page - db) / size_of::<Page>();
    }

    fn get_address(&self) -> PhysAddr {
        (self.get_pn() << arch::memory::get_page_bits()).into()
    }

    fn add_to_free_list(&mut self) {
        let order = self.current_order as usize;
        let mut pmm = PMM.lock();

        self.next = pmm.free_list[order];
        pmm.free_list[order] = NonNull::new(self);

        pmm.free_page_count[order] += 1;
    }

    fn pop_from_free_list(order: usize) -> Option<NonNull<Page>> {
        let mut pmm = PMM.lock();

        if pmm.free_list[order].is_none() {
            return None;
        }

        // Jump to the next entry.
        let page = pmm.free_list[order];
        unsafe {
            let p = page.unwrap().as_ptr();
            (*p).on_free_list = false;
            pmm.free_list[order] = (*p).next;
        }

        pmm.free_page_count[order] -= 1;

        return page;
    }
}

pub struct Buddy {
    free_list: [Option<NonNull<Page>>; NUM_BUDDIES],
    free_page_count: [usize; NUM_BUDDIES],
}

const NUM_BUDDIES: usize = 32;

pub static PMM: Mutex<Buddy> = Mutex::new(Buddy {
    free_list: [None; NUM_BUDDIES],
    free_page_count: [0; NUM_BUDDIES],
});

impl Buddy {
    fn order_from_size(size: usize) -> usize {
        assert!(size >= arch::memory::get_page_size(VmLevel::L1));

        return (size.trailing_zeros() as usize - arch::memory::get_page_bits())
            .min(NUM_BUDDIES - 1);
    }

    fn size_from_order(order: usize) -> usize {
        return 1usize << (order + arch::memory::get_page_bits());
    }

    fn free_region(start: PhysAddr, length: usize) {
        let mut page_db = PAGE_DB.lock();
        for offset in (0..length).step_by(arch::memory::get_page_size(VmLevel::L1)) {
            let page = page_db
                .get_mut(Page::idx_from_addr(start + offset))
                .unwrap();

            // Determine the highest order we can allocate in this region.
            let mut order = (page.get_pn().trailing_zeros() as usize).min(NUM_BUDDIES - 1);

            while order + Self::size_from_order(order) > length {
                order -= 1;
            }

            page.max_order = order as u8;
            page.current_order = order as u8;
            page.on_free_list = false;
        }

        let mut offset = 0;
        while offset < length {
            let page = page_db
                .get_mut(Page::idx_from_addr(start + offset))
                .unwrap();
            page.on_free_list = true;
            offset += Self::size_from_order(page.current_order as usize);
            page.add_to_free_list();
        }
    }

    fn alloc_page(mut order: usize) -> Option<NonNull<Page>> {
        assert!(order < NUM_BUDDIES);

        let target_order = order;

        {
            let pmm = PMM.lock();
            while pmm.free_list[order].is_none() {
                if order == NUM_BUDDIES - 1 {
                    return None;
                }
                order += 1;
            }
        }

        while order != target_order {
            let page = Page::pop_from_free_list(order).unwrap().as_ptr();
            let buddy = page.wrapping_add((1 << order) / 2);
            unsafe {
                assert!((*page).current_order == order as u8);
                assert!(
                    (*buddy).current_order == order as u8 - 1,
                    "Expected buddy for order {} to have order {}, but it was {}",
                    order,
                    order - 1,
                    (*buddy).current_order
                );

                (*page).current_order -= 1;

                (page.as_mut().unwrap()).add_to_free_list();
                (buddy.as_mut().unwrap()).add_to_free_list();
            }

            order -= 1;
        }

        let page = Page::pop_from_free_list(order);

        return page;
    }
}

impl PageAllocator for Buddy {
    fn alloc(pages: usize, flags: AllocFlags) -> Result<PhysAddr, AllocError> {
        let size = pages * arch::memory::get_page_size(VmLevel::L1);
        let order = Self::order_from_size(size);

        let page = Self::alloc_page(order).ok_or(AllocError)?;
        let addr = unsafe { page.as_ref() }.get_address();
        if flags.contains(AllocFlags::Zeroed) {
            unsafe { write_bytes(addr.as_hhdm() as *mut u8, 0, size) };
        }
        return Ok(addr);
    }

    unsafe fn dealloc(_addr: PhysAddr, _pages: usize) {
        // TODO
    }
}

/// Initializes the phyiscal memory manager.
pub fn init(memory_map: &[PhysMemory], pages: (*mut Page, usize)) {
    PAGE_DB_START.store(pages.0 as _, Ordering::Release);
    *PAGE_DB.lock() = unsafe { slice::from_raw_parts_mut(pages.0, pages.1) };

    // Register free regions.
    for entry in memory_map.iter() {
        Buddy::free_region(entry.address, entry.length);
    }

    // Count available memory.
    let mut total_memory = 0;
    let pmm = PMM.lock();

    for order in 0..NUM_BUDDIES {
        total_memory += pmm.free_page_count[order] * Buddy::size_from_order(order);
    }

    log!("Total available memory: {} KiB", total_memory / 1024);
}
