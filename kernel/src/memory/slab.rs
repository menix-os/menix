// Slab allocator

use super::pm::CommonPhysManager;
use crate::{arch::pm, thread::spin::SpinLock};
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
};

#[derive(Debug)]
struct Slab {
    /// Access lock.
    lock: SpinLock,
    /// Size of one entry.
    ent_size: usize,

    head: *mut *mut u8,
}

impl Slab {
    const fn empty() -> Self {
        Self {
            lock: SpinLock::new(),
            ent_size: 0,
            head: null_mut(),
        }
    }

    unsafe fn new(size: usize) -> Self {
        let mut slab = Self::empty();
        slab.head = pm::get_phys_base().byte_add(pm::alloc(1) as usize) as *mut *mut u8;

        return slab;
    }
}

// Storage for the slabs.
pub fn init() {
    unsafe {
        let slabs = [
            Slab::new(8),
            Slab::new(16),
            Slab::new(32),
            Slab::new(64),
            Slab::new(128),
            Slab::new(256),
            Slab::new(512),
            Slab::new(1024),
        ];
        SLAB_ALLOCATOR.slabs = slabs;
    }
}

#[repr(C, align(4096))]
struct SlabAllocator {
    slabs: [Slab; 8],
}

unsafe impl Sync for SlabAllocator {}

unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() == 0 {
            return null_mut();
        }

        return null_mut();
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}

// Register the slab allocator as the global allocator.
#[global_allocator]
static mut SLAB_ALLOCATOR: SlabAllocator = SlabAllocator {
    slabs: [
        Slab::empty(),
        Slab::empty(),
        Slab::empty(),
        Slab::empty(),
        Slab::empty(),
        Slab::empty(),
        Slab::empty(),
        Slab::empty(),
    ],
};
