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
    pub count: usize,
}
static_assert!(size_of::<Page>() <= 16);
static_assert!(0x1000 % size_of::<Page>() == 0);

pub static PAGE_DB: Mutex<&'static mut [Page]> = Mutex::new(&mut []);
pub static PAGE_DB_START: AtomicPtr<()> = AtomicPtr::new(null_mut());

pub static PMM: Mutex<Option<NonNull<Page>>> = Mutex::new(None);

pub struct FreeList;
impl PageAllocator for FreeList {
    fn alloc(pages: usize, flags: AllocFlags) -> Result<PhysAddr, AllocError> {
        let mut head = PMM.lock();
        let bytes = pages * arch::memory::get_page_size(VmLevel::L1);

        let mut addr = None;
        let mut it = *head;
        let mut prev_it = None;
        while let Some(mut x) = it {
            let page = unsafe { x.as_mut() };
            if page.count < pages {
                prev_it = it;
                it = page.next;
                continue;
            }

            if page.count == pages {
                addr = Some(page.get_address());
                if let Some(mut prev) = prev_it {
                    let prev_page = unsafe { prev.as_mut() };
                    prev_page.next = page.next;
                } else {
                    *head = page.next;
                }
                page.next = None;
                page.count = 0;
            } else {
                page.count -= pages;
                addr = Some(
                    page.get_address() + page.count * arch::memory::get_page_size(VmLevel::L1),
                );
            }
            break;
        }

        // TODO: Merge adjacent regions if we didn't find anything.
        assert!(addr.is_some());

        if flags.contains(AllocFlags::Zeroed) {
            unsafe { write_bytes(addr.unwrap().as_hhdm() as *mut u8, 0, bytes) };
        }

        return addr.ok_or(AllocError);
    }

    unsafe fn dealloc(addr: PhysAddr, pages: usize) {
        let mut head = PMM.lock();
        let mut page_db = PAGE_DB.lock();
        let page = page_db.get_mut(Page::idx_from_addr(addr)).unwrap();

        assert!(page.count == 0);
        assert!(page.next.is_none());

        page.count = pages;
        page.next = *head;
        *head = NonNull::new(page);
    }
}

impl Page {
    #[inline]
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
}

/// Initializes the phyiscal memory manager.
pub fn init(memory_map: &[PhysMemory], pages: (*mut Page, usize)) {
    PAGE_DB_START.store(pages.0 as _, Ordering::Release);
    *PAGE_DB.lock() = unsafe { slice::from_raw_parts_mut(pages.0, pages.1) };

    let mut total_memory = 0;

    // Register free regions.
    for entry in memory_map.iter() {
        let mut pmm = PMM.lock();
        let mut page_db = PAGE_DB.lock();
        let page = page_db.get_mut(Page::idx_from_addr(entry.address)).unwrap();
        page.count = entry.length / arch::memory::get_page_size(VmLevel::L1);
        page.next = *pmm;
        *pmm = NonNull::new(page);

        total_memory += entry.length;
    }

    log!("Total available memory: {} KiB", total_memory / 1024);
}
