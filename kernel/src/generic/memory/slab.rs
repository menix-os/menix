// Slab allocator

use super::{
    VirtAddr,
    buddy::BuddyAllocator,
    page::{AllocFlags, PageAllocator},
};
use crate::{
    arch::virt::PageTableEntry,
    generic::misc::{align_down, align_up},
};
use core::{
    alloc::{GlobalAlloc, Layout},
    hint::unlikely,
    mem::size_of,
    ptr::{null, null_mut, write_bytes},
};
use spin::Mutex;

#[derive(Debug)]
struct Slab {
    /// Size of one entry.
    ent_size: usize,
    head: Mutex<VirtAddr>,
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
            head: Mutex::new(VirtAddr::null()),
        }
    }

    /// Initializes a slab.
    fn init(&self) {
        unsafe {
            // Calculate the amount of bytes we need to skip in order to be able to store a reference to the slab.
            let offset = align_up(size_of::<SlabHeader>(), self.ent_size);
            // That also means we need to deduct that amount here.
            let available_size = (PageTableEntry::get_page_size()) - offset;

            // Allocate memory for this slab.
            let mem = BuddyAllocator::alloc(1, AllocFlags::Zeroed).expect("Out of memory");
            let mut head = mem.as_hhdm::<*mut ()>();

            // Get a reference to the start of the buffer.
            let ptr = head as *mut SlabHeader;
            // In that first entry, record a pointer to the head.
            (*ptr).slab = &raw const *self;
            // Now save that start to the slab.
            head = (head).byte_add(offset);

            let arr = head;
            let max = available_size / self.ent_size - 1;
            let fact = self.ent_size / size_of::<*mut ()>();

            for i in 0..max {
                *arr.add(i * fact) = arr.add((i + 1) * fact) as *mut ();
            }
            *arr.add(max * fact) = null_mut();

            *self.head.lock() = head.into();
        }
    }

    fn alloc(&self) -> *mut u8 {
        // Initialize the slab if not done already.
        if unlikely(*self.head.lock() == VirtAddr::null()) {
            self.init();
        }

        {
            let mut head = self.head.lock();
            let old_free = head.inner() as *mut *mut ();
            unsafe {
                *head = (*old_free).into();
                // Zero out the new allocation.
                write_bytes(old_free as *mut u8, 0, self.ent_size);
            }
            return old_free as *mut u8;
        }
    }
}

fn find_size(size: usize) -> Option<&'static Slab> {
    for slab in unsafe { &ALLOCATOR.slabs } {
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
            // TODO: This is broken.
            // let result = s.alloc();
            // assert!(result as usize % layout.align() == 0);
            // return result;
        }

        // The allocation won't fit within our defined slabs.
        // Get how many pages have to be allocated in order to fit `size`.
        let num_pages = align_up(layout.size(), PageTableEntry::get_page_size())
            / PageTableEntry::get_page_size();

        // Allocate the pages plus an additional page for metadata.
        match BuddyAllocator::alloc(num_pages + 1, AllocFlags::Zeroed) {
            Ok(mem) => unsafe {
                // Convert the physical address to a pointer.
                let ret: *mut u8 = mem.as_hhdm();

                // Write metadata into the first page.
                let info = ret as *mut SlabInfo;
                (*info).num_pages = num_pages;
                (*info).size = layout.size();

                // Skip the metadata and return the next one.
                return ret.byte_add(PageTableEntry::get_page_size());
            },
            Err(_) => return null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        return;
        if ptr == null_mut() {
            return;
        }

        if ptr as usize == align_down(ptr as usize, PageTableEntry::get_page_size()) {
            unsafe {
                let info = ptr.sub(PageTableEntry::get_page_size()) as *mut SlabInfo;
                BuddyAllocator::dealloc(info.into(), (*info).num_pages);
            }
        } else {
            unsafe {
                let header =
                    align_down(ptr as usize, PageTableEntry::get_page_size()) as *mut SlabHeader;
                //TODO: (*(*header).slab).free();
            }
        }
    }
}

#[repr(C, align(4096))]
pub(crate) struct SlabAllocator {
    slabs: [Slab; 10],
}

// Register the slab allocator as the global allocator.
#[global_allocator]
pub(crate) static ALLOCATOR: SlabAllocator = SlabAllocator {
    slabs: [
        Slab::new(16),
        Slab::new(24),
        Slab::new(32),
        Slab::new(48),
        Slab::new(64),
        Slab::new(128),
        Slab::new(256),
        Slab::new(512),
        Slab::new(1024),
        Slab::new(2048),
    ],
};
