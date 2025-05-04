use buddy::{PageNumber, Region};
use bump::BumpAllocator;
use page::Page;
use slab::ALLOCATOR;
use spin::Mutex;
use virt::{PageTable, VmFlags, VmLevel};

use crate::{
    arch,
    generic::{boot::BootInfo, util::align_up},
};
use core::{
    alloc::{GlobalAlloc, Layout},
    ops::{Add, Sub},
    ptr::{self, NonNull},
    sync::atomic::{AtomicUsize, Ordering},
};

pub mod buddy;
// We don't want to use the bump allocator anywhere after initial setup.
mod bump;
pub mod mmio;
pub mod page;
pub mod slab;
pub mod user;
pub mod virt;

static HHDM_START: AtomicUsize = AtomicUsize::new(0);

/// Global array that spans all usable physical memory.
/// It contains important metadata about a certain page.
/// This is virtually continuous, but not completely mapped in.
static PAGE_METADATA: Mutex<&[Page]> = Mutex::new(&[]);

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

/// Bootstraps the memory allocators and kernel virtual page table.
///
/// # Safety
///
/// Must be called as soon as control is handed to [`crate::main`] or the system won't function!
#[deny(dead_code)]
pub unsafe fn init() {
    let info = BootInfo::get();

    let kernel_phys = info
        .kernel_phys
        .expect("Kernel physical address should have been set");

    let paging_level = info
        .paging_level
        .expect("Paging level should have been set");

    let hhdm_address = info
        .hhdm_address
        .expect("HHDM address should have been set");

    HHDM_START.store(hhdm_address.value(), Ordering::Relaxed);

    let mut memory_map = info.memory_map.lock();

    // Remove 16-bit memory from the memory map.
    for entry in memory_map.iter_mut() {
        // Ignore 16-bit memory. This is 64KiB at most, and is required on some architectures like x86.
        if entry.address().value() < 1 << 16 {
            log!(
                "Ignoring 16-bit memory at {:#018x}",
                entry.address().value()
            );
            // If the entry is longer than 64KiB, shrink it in place. If it's not, completely ignore the entry.
            if entry.address() + entry.length() >= PhysAddr(1 << 16) {
                entry.set_length(entry.length() - (1 << 16) - entry.address().value());
                entry.set_address(PhysAddr(1 << 16));
            } else {
                continue;
            }
        }
    }

    // Find the highest usable memory.
    let highest_addr = memory_map
        .iter()
        .map(|x| (x.address().value() + x.length()))
        .max()
        .unwrap();

    // Calculate range of usable memory.
    let first_page = memory_map
        .iter()
        .map(|x| x.address().value() / arch::memory::get_page_size(VmLevel::L1))
        .next()
        .unwrap();

    let last_page = memory_map
        .iter()
        .map(|x| {
            align_up(
                x.address().value() + x.length(),
                arch::memory::get_page_size(VmLevel::L1),
            ) / arch::memory::get_page_size(VmLevel::L1)
        })
        .last()
        .unwrap();

    // Find the largest region there is.
    let bump_region = memory_map
        .iter()
        .max_by(|x, y| x.length().cmp(&y.length()))
        .unwrap();

    // Use that to bootstrap the bump allocator.
    bump::BUMP_CURRENT.store(
        align_up(
            bump_region.address().value(),
            arch::memory::get_page_size(VmLevel::L1),
        ),
        Ordering::Relaxed,
    );

    // Now we can start making page allocations!
    // ------------------------------------

    // Remap the kernel in our own page table.
    unsafe {
        log!("Using {}-level paging for page table", paging_level);
        let mut table = PageTable::new_kernel::<BumpAllocator>(paging_level);

        let text_start = VirtAddr(&raw const LD_TEXT_START as usize);
        let text_end = VirtAddr(&raw const LD_TEXT_END as usize);
        let rodata_start = VirtAddr(&raw const LD_RODATA_START as usize);
        let rodata_end = VirtAddr(&raw const LD_RODATA_END as usize);
        let data_start = VirtAddr(&raw const LD_DATA_START as usize);
        let data_end = VirtAddr(&raw const LD_DATA_END as usize);
        let kernel_start = VirtAddr(&raw const LD_KERNEL_START as usize);

        table
            .map_range::<BumpAllocator>(
                text_start,
                PhysAddr(text_start.0 - kernel_start.0 + kernel_phys.0),
                VmFlags::Read | VmFlags::Exec,
                VmLevel::L1,
                text_end.0 - text_start.0,
            )
            .expect("Unable to map the text segment");
        log!("Mapped text segment at {:#018x}", text_start.0);

        table
            .map_range::<BumpAllocator>(
                rodata_start,
                PhysAddr(rodata_start.0 - kernel_start.0 + kernel_phys.0),
                VmFlags::Read,
                VmLevel::L1,
                rodata_end.0 - rodata_start.0,
            )
            .expect("Unable to map the rodata segment");
        log!("Mapped rodata segment at {:#018x}", rodata_start.0);

        table
            .map_range::<BumpAllocator>(
                data_start,
                PhysAddr(data_start.0 - kernel_start.0 + kernel_phys.0),
                VmFlags::Read | VmFlags::Write,
                VmLevel::L1,
                data_end.0 - data_start.0,
            )
            .expect("Unable to map the data segment");
        log!("Mapped data segment at {:#018x}", data_start.0);

        // Map physical memory.
        table
            .map_range::<BumpAllocator>(
                hhdm_address,
                PhysAddr::null(),
                VmFlags::Read | VmFlags::Write,
                VmLevel::L3,
                128 * 1024 * 1024 * 1024,
            )
            .expect("Unable to map HHDM region");
        log!("Mapped HHDM segment at {:#018x}", hhdm_address.0);

        // Activate the new page table.
        table.set_active();

        // Save the page table.
        let mut kernel_table = virt::KERNEL_PAGE_TABLE.lock();
        *kernel_table = table;

        log!("Kernel map is now active");
    }

    let mut total_pages = 0;
    let mut actual_pages = 0;

    // Initialize all regions.
    for entry in memory_map.iter_mut() {
        let num_pages = align_up(entry.length(), arch::memory::get_page_size(VmLevel::L1))
            / arch::memory::get_page_size(VmLevel::L1);

        // Ignore memory regions which are too small to keep track of. We reserve at least one page for metadata.
        if num_pages < 2 {
            log!(
                "Ignoring single page region at {:#018x}",
                entry.address().value(),
            );
            continue;
        }

        let region = Region::new(
            (entry.address().value() + HHDM_START.load(Ordering::Relaxed)).into(),
            entry.address(),
            num_pages as PageNumber,
        );

        // Regions have to install metadata, which shrinks the amount of usable memory.
        actual_pages += region.get_num_pages();
        total_pages += num_pages;

        region.register();
    }

    log!(
        "Using {} KiB for page metadata, effective usable memory: {} MiB",
        (total_pages - actual_pages) * arch::memory::get_page_size(VmLevel::L1) / 1024,
        (actual_pages * (arch::memory::get_page_size(VmLevel::L1) - size_of::<Page>()))
            / 1024
            / 1024
    );

    // Set the MMAP base to right after the HHDM. Make sure this lands on a new PTE so we can map regular pages.
    let pte_size = arch::memory::get_page_size(VmLevel::L3);
    let offset = align_up(0x1000_0000_0000, pte_size);
    virt::KERNEL_MMAP_BASE_ADDR.store(hhdm_address.0 + offset, Ordering::Relaxed);
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

            pub const fn value(&self) -> usize {
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut core::ffi::c_void {
    let mem = unsafe { ALLOCATOR.alloc(Layout::from_size_align(size, align_of::<u8>()).unwrap()) };
    mem as *mut core::ffi::c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut core::ffi::c_void, size: usize) {
    unsafe {
        ALLOCATOR.dealloc(
            ptr as *mut u8,
            Layout::from_size_align(size, align_of::<u8>()).unwrap(),
        )
    };
}
