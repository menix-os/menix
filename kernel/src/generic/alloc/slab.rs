// Slab allocator

use super::{
    phys,
    virt::{self, KERNEL_PAGE_TABLE},
};
use crate::{
    arch::{PhysAddr, VirtAddr, virt::PageTableEntry},
    generic::misc::{align_down, align_up},
};
use core::{
    alloc::{GlobalAlloc, Layout},
    mem::size_of,
    ptr::{null_mut, write_bytes},
};
use spin::Mutex;

#[derive(Debug)]
struct Slab {
    /// Size of one entry.
    ent_size: usize,
    head: Mutex<Option<VirtAddr>>,
}

#[repr(transparent)]
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
            // Calculate the amount of bytes we need to skip in order to be able to store a reference to the slab.
            let offset = align_up(size_of::<SlabHeader>(), self.ent_size);
            // That also means we need to deduct that amount here.
            let available_size = (1 << PageTableEntry::get_page_bits()) - offset;

            // Allocate memory for this slab.
            let mem = phys::alloc(1).expect("Out of memory");
            let mut head = phys::direct_access(mem) as *mut *mut u8;

            // Get a reference to the start of the buffer.
            let ptr = head as *mut SlabHeader;
            // In that first entry, record a pointer to the head.
            (*ptr).slab = &raw const *self;
            // Now save that start to the slab.
            *self.head.lock() = Some(head.byte_add(offset) as VirtAddr);

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
        // Initialize the slab if not done already.
        if self.head.lock().is_none() {
            self.init();
        }

        {
            let mut head = self.head.lock();
            let old_free = head.unwrap() as *mut *mut u8;
            unsafe {
                *head = Some((*old_free) as VirtAddr);
                write_bytes(old_free, 0, self.ent_size);
            }
            return old_free as *mut u8;
        }
    }
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
            // TODO: The actual slab allocation is broken.
            //return s.alloc();
        }

        // The allocation won't fit within our defined slabs.
        // Get how many pages have to be allocated in order to fit `size`.
        let page_size = 1 << PageTableEntry::get_page_bits();
        let num_pages = align_up(layout.size(), page_size) / page_size;

        // Allocate the pages plus an additional page for metadata.
        match phys::alloc(num_pages + 1) {
            Some(mem) => unsafe {
                // Convert the physical address to a pointer.
                let ret = phys::direct_access(mem as PhysAddr);

                // Write metadata into the first page.
                let info = ret as *mut SlabInfo;
                (*info).num_pages = num_pages;
                (*info).size = layout.size();

                // Skip the metadata and return the next one.
                return ret.byte_add(page_size);
            },
            None => return null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ptr == null_mut() {
            return;
        }

        if ptr as VirtAddr == align_down(ptr as VirtAddr, 1 << PageTableEntry::get_page_bits()) {
            unsafe {
                let info = ptr.sub(1 << PageTableEntry::get_page_bits()) as *mut SlabInfo;
                phys::free(
                    info as VirtAddr - PageTableEntry::get_hhdm_addr(),
                    (*info).num_pages,
                );
            }
        } else {
            unsafe {
                let header = align_down(ptr as VirtAddr, 1 << PageTableEntry::get_page_bits())
                    as *mut SlabHeader;
                // TODO:
                //(*(*header).slab).free();
            }
        }
    }
}

#[repr(C, align(4096))]
struct SlabAllocator {
    slabs: [Slab; 8],
}

// Register the slab allocator as the global allocator.
#[global_allocator]
static SLAB_ALLOCATOR: SlabAllocator = SlabAllocator {
    slabs: [
        Slab::new(16),
        Slab::new(32),
        Slab::new(64),
        Slab::new(128),
        Slab::new(256),
        Slab::new(512),
        Slab::new(1024),
        Slab::new(2048),
    ],
};
