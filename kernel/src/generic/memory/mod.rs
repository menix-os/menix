use crate::{
    arch::virt::PageTableEntry,
    generic::{
        boot::{BootInfo, PhysMemoryUsage},
        misc,
    },
};
use alloc::alloc::{AllocError, Allocator};
use core::{
    ptr::{self, NonNull},
    sync::atomic::{AtomicUsize, Ordering},
};
use phys::{Page, PageNumber, Region};
use spin::Mutex;

pub mod heap;
mod libc;
pub mod mmio;
pub mod phys;
pub mod virt;

static HHDM_START: AtomicUsize = AtomicUsize::new(0);

/// Represents a physical address. It can't be directly read from or written to.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub usize);

impl PhysAddr {
    fn as_hhdm<T>(self) -> *mut T {
        VirtAddr(self.0 + HHDM_START.load(Ordering::Relaxed)).as_ptr()
    }
}

/// Represents a virtual address. It can't be directly read from or written to.
/// Note: Not the same as a pointer. A `VirtAddr` might point into another
/// process's memory that is not mapped in the kernel.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub usize);

impl<T> From<*const T> for VirtAddr {
    fn from(ptr: *const T) -> Self {
        Self(ptr as usize)
    }
}

impl<T> From<*mut T> for VirtAddr {
    fn from(ptr: *mut T) -> Self {
        Self(ptr as usize)
    }
}

impl<T> From<NonNull<T>> for VirtAddr {
    fn from(ptr: NonNull<T>) -> Self {
        Self(ptr.as_ptr() as usize)
    }
}

impl VirtAddr {
    pub fn as_ptr<T>(self) -> *mut T {
        return ptr::with_exposed_provenance_mut(self.0);
    }

    /// Returns the physical address mapped in the kernel for this [`VirtAddr`].
    pub fn as_hhdm(self) -> Option<PhysAddr> {
        return self
            .0
            .checked_sub(HHDM_START.load(Ordering::Relaxed))
            .map(PhysAddr);
    }
}

/// Initializes all memory management. Prior to this call, no allocations may be made.
#[deny(dead_code)]
pub(crate) fn init() {
    let info = BootInfo::get();

    let hhdm_address = info
        .hhdm_address
        .expect("HHDM address should have been set!");

    HHDM_START.store(hhdm_address.0, Ordering::Relaxed);

    let paging_level = info
        .paging_level
        .expect("Paging level should have been set!");

    let kernel_phys = info
        .kernel_phys
        .expect("Kernel physical address should have been set!");

    let kernel_virt = info
        .kernel_virt
        .expect("Kernel virtual address should have been set!");

    // Calculate range of usable memory.
    let first_page = info
        .memory_map
        .iter()
        .filter(|&f| f.usage == PhysMemoryUsage::Free)
        .map(|x| x.address.0 / PageTableEntry::get_page_size())
        .next()
        .unwrap();

    let last_page = info
        .memory_map
        .iter()
        .map(|x| (x.address.0 + x.length).div_ceil(PageTableEntry::get_page_size()))
        .last()
        .unwrap();

    let mut total_pages = 0;
    let mut actual_pages = 0;

    // Initialize all regions.
    for entry in info.memory_map {
        // Only care about usable memory.
        // TODO: Also consider reclaimable memory at some point.
        if entry.usage != PhysMemoryUsage::Free {
            continue;
        }

        // Copy this by value so we can do in-place modifications.
        let mut entry = *entry;

        // Ignore 16-bit memory. This is 64KiB at most, and is required on some architectures like x86.
        if entry.address.0 < 1 << 16 {
            print!(
                "memory: Ignoring 16-bit memory at {:#018x}\n",
                entry.address.0
            );
            // If the entry is longer than 64KiB, shrink it in place. If it's not, completely ignore the entry.
            if entry.address.0 + entry.length >= 1 << 16 {
                entry.length -= (1 << 16) - entry.address.0;
                entry.address = PhysAddr(1 << 16);
            } else {
                continue;
            }
        }

        let num_pages = misc::align_up(entry.length, PageTableEntry::get_page_size())
            / PageTableEntry::get_page_size();
        let meta_size = num_pages * size_of::<Page>();

        // Ignore memory regions which are too small to keep track of. We reserve at least one page for metadata.
        if num_pages < 2 {
            print!(
                "memory: Ignoring single page region at {:#018x}\n",
                entry.address.0,
            );
            continue;
        }

        let region = Region::new(
            VirtAddr(entry.address.0 + hhdm_address.0),
            PhysAddr(entry.address.0),
            num_pages as PageNumber,
        );

        // Regions have to install metadata, which shrinks the amount of usable memory.
        actual_pages += region.get_num_pages();
        total_pages += num_pages;

        region.register();
    }

    print!(
        "memory: Using {} KiB for page metadata, effective usable memory: {} MiB\n",
        (total_pages - actual_pages) * PageTableEntry::get_page_size() / 1024,
        (actual_pages * (PageTableEntry::get_page_size() - size_of::<Page>())) / 1024 / 1024
    );

    // TODO: Initialize the heap allocator.

    // Load our own page map.
    virt::init(
        hhdm_address,
        last_page * PageTableEntry::get_page_size(),
        paging_level,
        kernel_phys,
        kernel_virt,
    );
}
