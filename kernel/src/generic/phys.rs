use core::ptr::write_bytes;

use spin::Mutex;

use crate::{
    arch::{self, PhysAddr, VirtAddr, virt::PageTableEntry},
    generic::{bitmap::BitMap, misc::align_up},
};

/// Describes how a memory region is used.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub enum PhysMemoryUsage {
    /// Free and usable memory.
    Free,
    /// Memory reserved by the System.
    Reserved,
    /// Used by boot loader structures.
    Bootloader,
    /// Kernel and modules are loaded here.
    Kernel,
    /// Unknown memory region.
    #[default]
    Unknown,
}

/// Describes a region of physical memory.
#[derive(Clone, Copy, Debug, Default)]
pub struct PhysMemory {
    /// Start address of the memory region.
    pub address: PhysAddr,
    /// Length of the memory region in bytes.
    pub length: usize,
    /// How this memory region is used.
    pub usage: PhysMemoryUsage,
}

impl PhysMemory {
    pub const fn new() -> Self {
        Self {
            address: 0,
            length: 0,
            usage: PhysMemoryUsage::Unknown,
        }
    }
}

/// Physical memory allocator.
pub struct PhysManager<'a> {
    /// Base address of memory region where physical pages are mapped 1:1
    phys_base: VirtAddr,
    /// Bit map which stores whether or not a page is used.
    bit_map: BitMap<'a>,
    /// Amount of total installed pages.
    num_pages: usize,
    /// Amount of total free pages.
    num_free_pages: usize,
    /// The last page marked as used by the allocator.
    last_page: usize,
}

static PMM: Mutex<PhysManager> = Mutex::new(PhysManager {
    phys_base: 0,
    bit_map: BitMap::new(),
    num_pages: 0,
    num_free_pages: 0,
    last_page: 0,
});

impl PhysManager<'_> {
    /// Initializes the physical memory manager. Prior to this call, there may not be any heap allocations!
    pub fn init(memory_map: &mut [PhysMemory], identity_base: VirtAddr) {
        let mut pmm: spin::MutexGuard<'_, PhysManager<'_>> = PMM.lock();
        pmm.phys_base = identity_base;

        // Check for the highest usable physical memory address, so we know how much memory to allocate for the bitmap.
        let mut highest = 0;
        for entry in memory_map.iter() {
            if entry.usage != PhysMemoryUsage::Free {
                continue;
            }

            // Record the last byte of the current region if its address is the highest yet.
            let region_end = entry.address + entry.length as PhysAddr;
            if region_end > highest {
                highest = region_end;
            }
        }

        // How big the bit map needs to be to store a bit for each pages.
        let page_size = 1 << PageTableEntry::get_page_bits();
        let map_size;
        unsafe {
            pmm.num_pages = highest as usize / page_size;
            map_size = align_up((pmm.num_pages / 8) + 1, page_size);
        }

        // Get a memory region large enough to contain the bitmap.
        for entry in memory_map.iter_mut() {
            if entry.usage != PhysMemoryUsage::Free {
                continue;
            }

            if entry.length >= map_size {
                unsafe {
                    pmm.bit_map =
                        BitMap::from_ptr((pmm.phys_base + entry.address) as *mut u8, map_size);
                }

                // The region where the bitmap is stored is inaccessible now.
                // We could mark an entire entry as used, but that would be extremely wasteful.
                // map_size is definitely page-aligned at this point.
                entry.address += map_size as PhysAddr;
                entry.length -= map_size;
                break;
            }
        }

        // Mark all pages as used.
        pmm.bit_map.fill(true);

        // Mark the actual free pages as unused.
        for entry in memory_map.iter() {
            if entry.usage != PhysMemoryUsage::Free {
                continue;
            }

            for page in (entry.address as usize / page_size)
                ..((entry.address as usize + entry.length) / page_size)
            {
                pmm.bit_map.set(page, false);
                pmm.num_free_pages += 1;
            }
        }

        print!(
            "pm: Initialized physical memory management, {} MiB of free memory available.\n",
            (pmm.num_free_pages * page_size) / (1024 * 1024)
        );
    }

    /// Attempts to find `amount` free, consecutive pages, starting at page `start`.
    pub fn get_free_pages(&mut self, amount: usize, start: usize) -> Option<PhysAddr> {
        for i in start..self.num_pages {
            // Loop until we find a free page.
            if self.bit_map.get(i).unwrap() {
                continue;
            }

            // After finding a single free page, check if there are more free pages.
            for k in i..(i + amount) {
                if self.bit_map.get(k).unwrap() {
                    continue;
                }
            }

            // If we got here, that means we have found a region with `amount` consecutive pages.
            // Then, mark all requested pages as used.
            for k in i..(i + amount) {
                self.bit_map.set(k, true);
            }

            // Record the last page so future allocations can be more efficient.
            self.last_page = i + amount;
            return Some((i * (1 << PageTableEntry::get_page_bits())) as PhysAddr);
        }

        // Out of memory.
        return None;
    }

    pub fn alloc(num_pages: usize) -> Option<PhysAddr> {
        let mut allocator = PMM.lock();
        let last = allocator.last_page;

        // Try to find a matching region of physical memory.
        let mem = allocator.get_free_pages(num_pages, last)?;

        // Mark the pages as allocated here too.
        allocator.num_free_pages -= num_pages;
        return Some(mem);
    }

    pub fn alloc_zeroed(num_pages: usize) -> Option<PhysAddr> {
        let addr = Self::alloc(num_pages)?;
        unsafe {
            write_bytes(
                Self::direct_access(addr),
                0,
                1 << PageTableEntry::get_page_bits(),
            );
        };
        return Some(addr);
    }

    pub fn alloc_bytes(bytes: usize) -> Option<PhysAddr> {
        let addr = Self::alloc((bytes / (1 << PageTableEntry::get_page_bits())) + 1)?;
        unsafe {
            write_bytes(
                Self::direct_access(addr),
                0,
                1 << PageTableEntry::get_page_bits(),
            );
        };
        return Some(addr);
    }

    pub fn free(addr: PhysAddr, num_pages: usize) {
        let mut allocator = PMM.lock();
        let addr = (addr as usize >> PageTableEntry::get_page_bits());

        // Check if all pages in the given space are used. If not, that means the values given are gibberish.
        for i in addr..(addr + num_pages) {
            if allocator.bit_map.get(i).is_none_or(|f| f == false) {
                return;
            }
        }

        // Clear the bits for the respective pages in the bit map.
        for i in addr..(addr + num_pages) {
            allocator.bit_map.set(i, false);
        }
    }

    pub unsafe fn direct_access(address: PhysAddr) -> *mut u8 {
        unsafe {
            return (PMM.lock().phys_base + address) as *mut u8;
        }
    }
}
