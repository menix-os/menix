use crate::{
    arch::virt::PageTableEntry,
    generic::{
        boot::BootInfo,
        misc::{self, align_up},
    },
};
use core::{
    ops::{Add, Sub},
    ptr::{self, NonNull},
    sync::atomic::{AtomicUsize, Ordering},
};
use phys::{Page, PageNumber, Region};

pub mod init;
pub mod mmio;
pub mod phys;
pub mod slab;
pub mod virt;

static HHDM_START: AtomicUsize = AtomicUsize::new(0);

/// Initializes all memory management. Prior to this call, no allocations may be made.
#[deny(dead_code)]
pub(crate) fn init() {
    let info = BootInfo::get();

    let hhdm_address = info
        .hhdm_address
        .expect("HHDM address should have been set!");

    HHDM_START.store(hhdm_address.inner(), Ordering::Relaxed);

    let paging_level = info
        .paging_level
        .expect("Paging level should have been set!");

    let kernel_phys = info
        .kernel_phys
        .expect("Kernel physical address should have been set!");

    let kernel_virt = info
        .kernel_virt
        .expect("Kernel virtual address should have been set!");

    let mut memory_map = info.memory_map.lock();

    // Calculate range of usable memory.
    let first_page = memory_map
        .iter()
        .map(|x| x.address().inner() / PageTableEntry::get_page_size())
        .next()
        .unwrap();

    let last_page = memory_map
        .iter()
        .map(|x| {
            align_up(
                x.address().inner() + x.length(),
                PageTableEntry::get_page_size(),
            ) / PageTableEntry::get_page_size()
        })
        .last()
        .unwrap();

    let mut total_pages = 0;
    let mut actual_pages = 0;

    // Initialize all regions.
    for entry in memory_map.iter_mut() {
        // Ignore 16-bit memory. This is 64KiB at most, and is required on some architectures like x86.
        if entry.address().inner() < 1 << 16 {
            print!(
                "memory: Ignoring 16-bit memory at {:#018X}\n",
                entry.address().inner()
            );
            // If the entry is longer than 64KiB, shrink it in place. If it's not, completely ignore the entry.
            if entry.address() + entry.length() >= PhysAddr(1 << 16) {
                entry.set_length(entry.length() - (1 << 16) - entry.address().inner());
                entry.set_address(PhysAddr(1 << 16));
            } else {
                continue;
            }
        }

        let num_pages = misc::align_up(entry.length(), PageTableEntry::get_page_size())
            / PageTableEntry::get_page_size();
        let meta_size = num_pages * size_of::<Page>();

        // Ignore memory regions which are too small to keep track of. We reserve at least one page for metadata.
        if num_pages < 2 {
            print!(
                "memory: Ignoring single page region at {:#018X}\n",
                entry.address().inner(),
            );
            continue;
        }

        let region = Region::new(
            (entry.address().inner() + hhdm_address.inner()).into(),
            entry.address(),
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

    print!(
        "memory: Using {}-level paging for page table.\n",
        paging_level
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

/// Represents a physical address. It can't be directly read from or written to.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(usize);

impl PhysAddr {
    pub fn as_hhdm<T>(self) -> *mut T {
        VirtAddr(self.0 + HHDM_START.load(Ordering::Relaxed)).as_ptr()
    }
}

/// Represents a virtual address. It can't be directly read from or written to.
/// Note: Not the same as a pointer. A `VirtAddr` might point into another
/// process's memory that is not mapped in the kernel.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(usize);

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

macro_rules! addr_impl {
    ($ty:ty) => {
        impl $ty {
            pub const fn null() -> Self {
                Self(0)
            }

            pub const fn new(value: usize) -> Self {
                Self(value)
            }

            pub const fn inner(&self) -> usize {
                self.0
            }
        }

        impl Into<usize> for $ty {
            fn into(self) -> usize {
                self.0
            }
        }
        #[cfg(target_pointer_width = "32")]
        impl Into<u32> for $ty {
            fn into(self) -> u32 {
                self.0 as u32
            }
        }

        #[cfg(target_pointer_width = "64")]
        impl Into<u64> for $ty {
            fn into(self) -> u64 {
                self.0 as u64
            }
        }

        impl From<usize> for $ty {
            fn from(addr: usize) -> Self {
                Self(addr)
            }
        }

        #[cfg(target_pointer_width = "32")]
        impl From<u32> for $ty {
            fn from(addr: u32) -> Self {
                Self(addr as usize)
            }
        }

        #[cfg(target_pointer_width = "64")]
        impl From<u64> for $ty {
            fn from(addr: u64) -> Self {
                Self(addr as usize)
            }
        }

        impl<T> From<*const T> for $ty {
            fn from(ptr: *const T) -> Self {
                Self(ptr as usize)
            }
        }

        impl<T> From<*mut T> for $ty {
            fn from(ptr: *mut T) -> Self {
                Self(ptr as usize)
            }
        }

        impl<T> From<NonNull<T>> for $ty {
            fn from(ptr: NonNull<T>) -> Self {
                Self(ptr.as_ptr() as usize)
            }
        }

        impl Add for $ty {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl Sub for $ty {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl Add<usize> for $ty {
            type Output = Self;

            fn add(self, rhs: usize) -> Self::Output {
                Self(self.0 + rhs)
            }
        }

        impl Sub<usize> for $ty {
            type Output = Self;

            fn sub(self, rhs: usize) -> Self::Output {
                Self(self.0 - rhs)
            }
        }
    };
}

addr_impl!(PhysAddr);
addr_impl!(VirtAddr);
