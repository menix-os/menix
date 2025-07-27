use super::PhysAddr;
use crate::{
    arch,
    generic::{
        boot::PhysMemory,
        memory::virt::VmLevel,
        util::{align_up, spin_mutex::SpinMutex},
    },
};
use alloc::alloc::AllocError;
use bitflags::bitflags;
use core::{
    hint::unlikely,
    ptr::{NonNull, null_mut, write_bytes},
    slice,
    sync::atomic::{AtomicPtr, Ordering},
};

bitflags! {
    #[derive(Debug)]
    pub struct AllocFlags: usize {
        /// Only consider physical memory below 1MiB.
        const Kernel20 = 1 << 0;
        /// Only consider physical memory below 4GiB.
        const Kernel32 = 1 << 1;
        /// Allocated memory has to be initialized to zero.
        const Zeroed = 1 << 2;
    }
}

pub trait PageAllocator {
    /// Allocates `pages` amount of consecutive pages.
    fn alloc(pages: usize, flags: AllocFlags) -> Result<PhysAddr, AllocError>;

    /// Allocates enough consecutive pages to fit `bytes` amount of bytes.
    fn alloc_bytes(bytes: usize, flags: AllocFlags) -> Result<PhysAddr, AllocError> {
        let pages = align_up(bytes, arch::virt::get_page_size(VmLevel::L1))
            / arch::virt::get_page_size(VmLevel::L1);
        return Self::alloc(pages, flags);
    }

    /// Deallocates a region of `pages` amount of consecutive pages.
    /// # Safety
    /// Deallocating arbitrary physical addresses is inherently unsafe, since it can cause the kernel to corrupt.
    unsafe fn dealloc(addr: PhysAddr, pages: usize);
}

// WARNING: Keep this structure as small as possible, every single physical page has one!
/// Metadata about a physical page.
#[derive(Debug)]
#[repr(C)]
pub struct Page {
    pub next: Option<NonNull<Page>>,
    pub count: usize,
}

// If this assert fails, the PFNDB can't properly allocate data.
static_assert!(0x1000 % size_of::<Page>() == 0);

pub static PAGE_DB: SpinMutex<&'static mut [Page]> = SpinMutex::new(&mut []);
pub static PAGE_DB_START: AtomicPtr<()> = AtomicPtr::new(null_mut());

pub static PMM: SpinMutex<Option<NonNull<Page>>> = SpinMutex::new(None);

pub struct KernelAlloc;
impl PageAllocator for KernelAlloc {
    fn alloc(pages: usize, flags: AllocFlags) -> Result<PhysAddr, AllocError> {
        let mut head = PMM.lock();
        let bytes = pages * arch::virt::get_page_size(VmLevel::L1);

        let limit = if flags.contains(AllocFlags::Kernel20) {
            PhysAddr(1 << 20)
        } else if flags.contains(AllocFlags::Kernel32) {
            PhysAddr(1 << 32)
        } else {
            PhysAddr(usize::MAX)
        };

        let mut addr = None;
        let mut it = *head;
        let mut prev_it = None;
        while let Some(mut x) = it {
            let page = unsafe { x.as_mut() };

            if page.get_address() + bytes >= limit {
                prev_it = it;
                it = page.next;
                continue;
            }

            if unlikely(page.count < pages) {
                prev_it = it;
                it = page.next;
                continue;
            }

            if unlikely(page.count == pages) {
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
                addr =
                    Some(page.get_address() + page.count * arch::virt::get_page_size(VmLevel::L1));
            }
            break;
        }

        // TODO: Merge adjacent regions if we didn't find anything.
        debug_assert!(addr.is_some());

        if flags.contains(AllocFlags::Zeroed) {
            unsafe { write_bytes(addr.unwrap().as_hhdm() as *mut u8, 0, bytes) };
        }

        return addr.ok_or(AllocError);
    }

    unsafe fn dealloc(addr: PhysAddr, pages: usize) {
        let mut head = PMM.lock();
        let mut page_db = PAGE_DB.lock();
        let page = page_db.get_mut(Page::idx_from_addr(addr)).unwrap();

        debug_assert!(page.count == 0);
        debug_assert!(page.next.is_none());

        page.count = pages;
        page.next = *head;
        *head = NonNull::new(page);
    }
}

impl Page {
    #[inline]
    pub fn idx_from_addr(address: PhysAddr) -> usize {
        address.0 / arch::virt::get_page_size(VmLevel::L1)
    }

    /// Returns the page number of this page.
    fn get_pn(&self) -> usize {
        let page: *const Page = self;
        let page = page as usize;
        let db = PAGE_DB_START.load(Ordering::Relaxed) as usize;
        debug_assert!(page >= db, "{page:#018x} >= {db:#018x}");
        return (page - db) / size_of::<Page>();
    }

    #[inline]
    fn get_address(&self) -> PhysAddr {
        (self.get_pn() << arch::virt::get_page_bits()).into()
    }
}

/// Initializes the phyiscal memory manager.
pub fn init(memory_map: &[PhysMemory], pages: (*mut Page, usize)) {
    PAGE_DB_START.store(pages.0 as _, Ordering::Release);
    *PAGE_DB.lock() = unsafe { slice::from_raw_parts_mut(pages.0, pages.1) };

    let mut total_memory = 0;

    // Register free regions.
    for entry in memory_map.iter() {
        if entry.length < arch::virt::get_page_size(VmLevel::L1) {
            continue;
        }

        let mut pmm = PMM.lock();
        let mut page_db = PAGE_DB.lock();
        let page = page_db.get_mut(Page::idx_from_addr(entry.address)).unwrap();
        page.count = entry.length / arch::virt::get_page_size(VmLevel::L1);
        page.next = *pmm;
        *pmm = NonNull::new(page);

        total_memory += entry.length;
    }

    log!("Total available memory: {} MiB", total_memory / 1024 / 1024);
}
