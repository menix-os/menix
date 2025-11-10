use crate::spec;
use menix::{
    memory::{AllocFlags, KernelAlloc, MmioView, PageAllocator},
    posix::errno::EResult,
};

pub struct Queue {
    queue_id: usize,
    depth: usize,
    doorbells: MmioView,
    cq_view: MmioView,
    sq_view: MmioView,
}

impl Queue {
    /// Creates a new submission and completion queue pair.
    pub fn new(queue_id: usize, depth: usize, doorbells: MmioView) -> EResult<Self> {
        let align = 0x1000;
        let sq_size = ((depth << 6) + align - 1) & !(align - 1);
        let cq_size = ((depth * spec::cq_entry::SIZE) + align - 1) & !(align - 1);

        // Allocate memory the completion queue.
        let cq_addr = KernelAlloc::alloc_bytes(cq_size, AllocFlags::Zeroed)?;
        let cq_view = unsafe { MmioView::new(cq_addr, cq_size) };

        // Allocate memory for the submission queue.
        let sq_addr = KernelAlloc::alloc_bytes(sq_size, AllocFlags::Zeroed)?;
        let sq_view = unsafe { MmioView::new(sq_addr, sq_size) };

        Ok(Self {
            queue_id,
            depth,
            doorbells,
            cq_view,
            sq_view,
        })
    }
}
