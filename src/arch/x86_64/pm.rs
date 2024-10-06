// Physical memory management

use alloc::{rc::Rc, sync::Arc};

use crate::{
    arch::{PhysAddr, PAGE_SIZE},
    boot::BootInfo,
    memory::pm::{CommonPhysManager, PhysMemoryUsage},
    misc::{align_up, bitmap::BitMap},
    thread::spin::SpinLock,
};
use core::{
    cell::{Cell, RefCell},
    ptr::null_mut,
};

pub struct PhysManager {
    /// Access lock for the manager.
    lock: SpinLock,
    /// Base address of memory region where physical pages are mapped 1:1
    phys_base: *mut u8,
    /// Bit map which stores whether or not a page is used.
    bit_map: BitMap,
    /// Amount of total installed pages.
    num_pages: usize,
    /// Amount of total free pages.
    num_free_pages: usize,
}

unsafe impl Sync for PhysManager {}
unsafe impl Send for PhysManager {}

static mut PMM: PhysManager = PhysManager {
    lock: SpinLock::new(),
    phys_base: null_mut(),
    bit_map: BitMap::new(),
    num_pages: 0,
    num_free_pages: 0,
};

impl CommonPhysManager for PhysManager {
    unsafe fn init(info: &mut BootInfo) {
        assert!(info.identity_base != 0, "HHDM base was NULL!");

        let pmm = unsafe { &raw mut PMM };

        unsafe {
            (*pmm).phys_base = info.identity_base as *mut u8;
        }

        // Check for the highest usable physical memory address, so we know how much memory to allocate for the bitmap.
        let mut highest = 0;
        for entry in info.memory_map.iter() {
            // Only care about memory that we are able to own.
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
            (*pmm).num_pages = highest as usize / PAGE_SIZE;
            map_size = align_up((*pmm).num_pages / 8, PAGE_SIZE);
        }

        // Get a memory region large enough to contain the bitmap.
        for entry in info.memory_map.iter_mut() {
            // Only care about memory that we are able to own.
            if entry.usage != PhysMemoryUsage::Free {
                continue;
            }

            if entry.length >= map_size {
                unsafe {
                    (*pmm).bit_map = BitMap::from_ptr(
                        PhysManager::phys_base().add(entry.address as usize),
                        map_size,
                    );
                }

                // The region where the bitmap is stored is inaccessible now.
                // We could mark an entire page as used, but that would be wasteful.
                entry.address += map_size as u64;
                entry.length -= map_size;
                break;
            }
        }

        // Mark all pages as used.
        unsafe {
            (*pmm).bit_map.fill(true);
        }

        // Mark the actual free pages as unused.
        // TODO: This is extremely slow! Fix me!
        for entry in info.memory_map.iter() {
            // Only care about memory that we are able to own.
            if entry.usage != PhysMemoryUsage::Free {
                continue;
            }

            for page in (entry.address as usize / PAGE_SIZE)
                ..((entry.address as usize + entry.length) / PAGE_SIZE)
            {
                unsafe {
                    (*pmm).bit_map.set(page, false);
                    (*pmm).num_free_pages += 1;
                }
            }
        }

        //println!("Initialized physical memory management, free memory = {} MiB\n",
        //		 (num_free_pages * PAGE_SIZE) / MIB);

        todo!();
    }

    unsafe fn alloc(num_pages: usize) -> PhysAddr {
        todo!();
    }

    unsafe fn free(addr: PhysAddr, num_pages: usize) {
        // TODO:
        // Check if all pages in the given space are used. If not, that means the values given are gibberish.
        // Clear the bits for the respective pages in the bit map.
        todo!();
    }

    unsafe fn phys_base() -> *mut u8 {
        unsafe {
            return PMM.phys_base;
        }
    }
}
