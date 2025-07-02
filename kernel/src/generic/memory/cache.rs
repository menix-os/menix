use crate::generic::{
    memory::{PhysAddr, virt::VmFlags},
    percpu::CpuData,
    process::{Process, task::Task},
    util::mutex::Mutex,
};
use alloc::{
    collections::btree_map::BTreeMap,
    sync::{Arc, Weak},
};
use core::fmt::Debug;

/// A virtual mapped memory object.
#[derive(Debug)]
pub struct Object {
    pub backing: Option<Arc<Object>>,
    /// All currently present pages of this object. Keys are offsets (in pages) to physical pages.
    pub present_pages: Mutex<BTreeMap<usize, PhysAddr>>,
    pub num_pages: usize,
    /// Used to read and write back pages from/to a backing storage.
    pub pager: Option<Weak<dyn Pager>>,
    /// The protection flags for this mapping.
    pub protection: VmFlags,
}

impl Object {
    /// Creates a new anonymous object.
    pub fn new_anon(num_pages: usize, protection: VmFlags) -> Self {
        // TODO
        Self {
            backing: None,
            present_pages: Mutex::new(BTreeMap::new()),
            num_pages,
            pager: None,
            protection,
        }
    }

    pub fn new_paged(num_pages: usize, protection: VmFlags, pager: Weak<dyn Pager>) -> Self {
        Self {
            backing: None,
            num_pages,
            present_pages: Mutex::new(BTreeMap::new()),
            pager: Some(pager),
            protection,
        }
    }
}

pub trait Pager: Debug {
    /// Reads pages from the backing storage.
    fn get_pages(
        &self,
        object: &Object,
        pages: &[usize],
        faulty_page: usize,
    ) -> Result<&[usize], PagerError>;
    /// Writes pages to the backing store.
    fn write_pages(&self, object: &Object, pages: &[u64]) -> Result<(), PagerError>;
}

pub enum PagerError {}

init_stage! {
    #[depends(crate::generic::process::sched::SCHEDULER_STAGE)]
    CACHE_STAGE: "generic.memory.cache" => init;
}

fn init() {
    let task = Task::new(cache_flush, 0, 0, Process::get_kernel(), false)
        .expect("Unable to start page cache task");
    CpuData::get().scheduler.add_task(Arc::new(task));
}

extern "C" fn cache_flush(_: usize, _: usize) {
    // TODO
}
