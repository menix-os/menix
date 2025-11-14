use crate::{command::Command, spec::CompletionEntry};
use menix::{
    alloc::sync::Arc,
    memory::{AllocFlags, KernelAlloc, MmioView, PageAllocator, PhysAddr},
    posix::errno::EResult,
};

pub struct Queue {
    queue_id: usize,
    depth: usize,
    view: Arc<MmioView>,
    cq_addr: PhysAddr,
    cq_view: MmioView,
    sq_addr: PhysAddr,
    sq_view: MmioView,
}

impl Queue {
    /// Creates a new submission and completion queue pair.
    pub fn new(queue_id: usize, depth: usize, view: Arc<MmioView>) -> EResult<Self> {
        let align = 0x1000;
        let sq_size = ((depth << 6) + align - 1) & !(align - 1);
        let cq_size = ((depth * (size_of::<CompletionEntry>())) + align - 1) & !(align - 1);
        // Allocate memory the completion queue.
        let cq_addr = KernelAlloc::alloc_bytes(cq_size as _, AllocFlags::Zeroed)?;
        let cq_view = unsafe { MmioView::new(cq_addr, cq_size as _) };

        // Allocate memory for the submission queue.
        let sq_addr = KernelAlloc::alloc_bytes(sq_size as _, AllocFlags::Zeroed)?;
        let sq_view = unsafe { MmioView::new(sq_addr, sq_size as _) };

        menix::log!("Created queue {queue_id}: sq_size = {sq_size}, cq_size = {cq_size}");

        Ok(Self {
            queue_id,
            depth,
            view,
            cq_view,
            sq_view,
            cq_addr,
            sq_addr,
        })
    }

    pub fn submit_cmd(&self, command: Command) {
        todo!()
    }

    pub fn get_sq_addr(&self) -> PhysAddr {
        self.sq_addr
    }

    pub fn get_cq_addr(&self) -> PhysAddr {
        self.cq_addr
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            KernelAlloc::dealloc_bytes(self.sq_addr, self.sq_view.len());
            KernelAlloc::dealloc_bytes(self.cq_addr, self.cq_view.len());
        }
    }
}
