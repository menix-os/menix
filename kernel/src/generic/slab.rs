// Slab allocator

use core::{
    alloc::{GlobalAlloc, Layout},
    mem::size_of,
    ops::Deref,
    ptr::{null, null_mut, write_bytes},
};
use spin::Mutex;

use crate::{arch, generic::misc::align_up};

use super::phys::PhysManager;

#[derive(Debug)]
struct Slab {
    /// Size of one entry.
    ent_size: usize,

    head: Mutex<Option<usize>>,
}

struct SlabHeader {
    slab: *const Slab,
}

struct SlabInfo {
    /// Amount of pages connected to this slab.
    num_pages: usize,
    /// Size of this slab.
    size: usize,
}

impl Slab {
    /// Creates a new, uninitialized slab.
    const fn new(size: usize) -> Self {
        Self {
            ent_size: size,
            head: Mutex::new(None),
        }
    }

    /// Initializes a slab.
    fn init(&self) {
        unsafe {
            // Allocate memory for this slab.
            let mut head = PhysManager::phys_base().byte_add(PhysManager::alloc(1) as usize);

            // Calculate the amount of bytes we need to skip in order to be able to store a reference to the slab.
            let offset = align_up(size_of::<SlabHeader>(), self.ent_size);
            // That also means we need to deduct that amount here.
            let available_size = arch::get_page_size() - offset;

            // Get a reference to the start of the buffer.
            let ptr = head as *mut SlabHeader;
            // In that first entry, record a pointer to the head.
            (*ptr).slab = &raw const *self;
            // Offset the start of the entries.
            head = head.byte_add(offset) as *mut u8;
            // Now save that start to the slab.
            *self.head.lock() = Some(head as usize);

            let arr = head as *mut *mut u8;
            let max = available_size / self.ent_size - 1;
            let fact = self.ent_size / size_of::<*mut u8>();

            for i in 0..max {
                *arr.add(i * fact) = arr.add((i + 1) * fact) as *mut u8;
            }
            *arr.add(max * fact) = null_mut();
        }
    }

    fn alloc(&self) -> *mut u8 {
        let mut head = self.head.lock();
        // Initialize the slab if not done already.
        if head.is_none() {
            drop(head);
            self.init();
        }

        head = self.head.lock();
        let old_free = head.unwrap() as *mut *mut u8;
        unsafe {
            *head = Some(*old_free as usize);
            write_bytes(old_free, 0, self.ent_size);
        }

        return old_free as *mut u8;
    }
}

#[repr(C, align(4096))]
struct SlabAllocator {
    slabs: [Slab; 8],
}

fn find_size(size: usize) -> Option<&'static Slab> {
    for slab in unsafe { &SLAB_ALLOCATOR.slabs } {
        if slab.ent_size >= size {
            return Some(slab);
        }
    }
    return None;
}

unsafe impl GlobalAlloc for SlabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // If there's nothing to allocate, don't.
        if layout.size() == 0 {
            return null_mut();
        }

        // Find a suitable slab.
        let slab = find_size(layout.size());
        if let Some(s) = slab {
            // The allocation fits within our defined slabs.
            return s.alloc();
        }

        // The allocation won't fit within our defined slabs.
        // Get how many pages have to be allocated in order to fit `size`.
        let page_size = arch::get_page_size();
        let num_pages = align_up(layout.size(), page_size) / page_size;

        // Allocate the pages plus an additional page for metadata.
        let mem = PhysManager::alloc(num_pages + 1);
        if mem == 0 {
            return null_mut();
        }

        unsafe {
            // Convert the physical address to a pointer.
            let ret = PhysManager::phys_base().add(mem as usize);

            // Write metadata into the first page.
            let info = ret as *mut SlabInfo;
            (*info).num_pages = num_pages;
            (*info).size = layout.size();

            // Skip the metadata and return the next one.
            return ret.byte_add(page_size);
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // TODO: do deallocation
    }
}

// Register the slab allocator as the global allocator.
#[global_allocator]
static SLAB_ALLOCATOR: SlabAllocator = SlabAllocator {
    slabs: [
        Slab::new(8),
        Slab::new(16),
        Slab::new(32),
        Slab::new(64),
        Slab::new(128),
        Slab::new(256),
        Slab::new(512),
        Slab::new(1024),
    ],
};
