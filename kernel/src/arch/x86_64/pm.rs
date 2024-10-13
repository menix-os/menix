// Physical memory management

use spin::Mutex;

use super::VirtAddr;
use crate::{
    arch::{PhysAddr, PAGE_SIZE},
    boot::BootInfo,
    dbg, log,
    memory::pm::{CommonPhysManager, PhysMemoryUsage},
    misc::{align_down, align_up, bitmap::BitMap, units::MiB},
    system::error::Errno,
};
use core::ptr::null_mut;

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
    /// Attempts to find `amount` free, consecutive pages, starting at page `start`.
    fn get_free_pages(&mut self, amount: usize, start: usize) -> Result<PhysAddr, Errno> {
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
            return Ok((i * PAGE_SIZE) as PhysAddr);
        }
        return Err(Errno::ENOMEM);
    }
}

impl CommonPhysManager for PhysManager<'_> {
    unsafe fn init(info: &mut BootInfo) {
        assert!(info.identity_base != 0, "HHDM base was NULL!");

        log!("Bootloader provided memory map:\n");
        log!("Index\tAddress\t\t\tSize\t\t\tUsage\n");
        for (i, entry) in info.memory_map.iter().enumerate() {
            log!(
                "[{i}]\t{:#018x}\t{:#018x}\t{:?}\n",
                entry.address,
                entry.length,
                entry.usage,
            );
        }

        let mut pmm: spin::MutexGuard<'_, PhysManager<'_>> = PMM.lock();
        pmm.phys_base = info.identity_base;

        // Check for the highest usable physical memory address, so we know how much memory to allocate for the bitmap.
        let mut highest = 0;
        for entry in info.memory_map.iter() {
            if entry.usage != PhysMemoryUsage::Free {
                continue;
            }

            // Record the last byte of the current region if its address is the highest yet.
            let region_end = entry.address + entry.length as u64;
            if region_end > highest {
                highest = region_end;
            }
        }

        // How big the bit map needs to be to store a bit for each pages.
        let map_size;
        unsafe {
            pmm.num_pages = highest as usize / PAGE_SIZE;
            map_size = align_up((pmm.num_pages / 8) + 1, PAGE_SIZE);
        }

        // Get a memory region large enough to contain the bitmap.
        for entry in info.memory_map.iter_mut() {
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
                entry.address += map_size as u64;
                entry.length -= map_size;
                break;
            }
        }

        // Mark all pages as used.
        pmm.bit_map.fill(true);

        // Mark the actual free pages as unused.
        for entry in info.memory_map.iter() {
            if entry.usage != PhysMemoryUsage::Free {
                continue;
            }

            for page in (entry.address as usize / PAGE_SIZE)
                ..((entry.address as usize + entry.length) / PAGE_SIZE)
            {
                pmm.bit_map.set(page, false);
                pmm.num_free_pages += 1;
            }
        }

        log!(
            "Initialized physical memory management, free memory = {} MiB\n",
            (pmm.num_free_pages * PAGE_SIZE) / MiB
        );
    }

    fn alloc(num_pages: usize) -> PhysAddr {
        let mut allocator = PMM.lock();
        let last = allocator.last_page;

        // Try to find a matching region of physical memory.
        let mem = match allocator.get_free_pages(num_pages, last) {
            Ok(x) => x,
            Err(_) => {
                // Last ditch effort to recover if an allocation has failed.
                // Start looking for free pages from the beginning again.
                // This is way slower than the previous call, but unavoidable.
                allocator
                    .get_free_pages(num_pages, 0)
                    .expect("Out of physical memory!")
            }
        };

        // Mark the pages as allocated here too.
        allocator.num_free_pages -= num_pages;
        return mem;
    }

    fn free(addr: PhysAddr, num_pages: usize) {
        let mut allocator = PMM.lock();
        let addr = (addr as usize / PAGE_SIZE);

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

    fn phys_base() -> *mut u8 {
        unsafe {
            return PMM.lock().phys_base as *mut u8;
        }
    }
}
